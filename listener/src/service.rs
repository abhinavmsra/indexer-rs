use std::{
    env,
    error::Error,
    future::Future,
    pin::Pin,
    str::FromStr,
    task::{Context, Poll},
};

use alloy::{
    eips::BlockNumberOrTag,
    primitives::Address,
    providers::{Provider, ProviderBuilder},
    rpc::types::Filter,
};
use indexer_db::entity::{evm_logs::EvmLogs, evm_sync_logs::EvmSyncLogs};
use sqlx::{Pool, Postgres};
use tower::Service;

use crate::error::AppError;

pub struct ListenerService {
    pub chain_id: u64,
    pub address: String,
    pub db_pool: Pool<Postgres>,
}

impl Service<()> for ListenerService {
    type Response = ();
    type Error = Box<dyn Error>;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, _: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, _: ()) -> Self::Future {
        let db_pool = self.db_pool.clone();
        let chain_id = self.chain_id;
        let address = self.address.clone();

        Box::pin(async move { fetch_and_save_logs(chain_id, db_pool, address).await })
    }
}

pub async fn fetch_and_save_logs(
    chain_id: u64,
    db_pool: Pool<Postgres>,
    address: String,
) -> Result<(), Box<dyn Error>> {
    let rpc_url = env::var("RPC_URL").map_err(|_| AppError::MissingEnvVar("RPC_URL".into()))?;

    let provider = ProviderBuilder::new().on_builtin(&rpc_url).await?;
    let sync_log = EvmSyncLogs::find_or_create_by_address(&address, chain_id, &db_pool).await?;

    let latest_block = provider.get_block_number().await?;
    if latest_block == sync_log.last_synced_block_number as u64 {
        println!("Fully indexed address: {address}");
        return Ok(());
    }

    let from_block_number = match sync_log.last_synced_block_number as u64 {
        0 => 0, // FIXME: may start from the first tx block
        block_number => block_number + 1_u64,
    };

    let to_block_number = match sync_log.last_synced_block_number as u64 {
        0 => latest_block,
        block_number => std::cmp::min(block_number + 10_000_u64, latest_block),
    };

    let filter = Filter::new()
        .address(Address::from_str(&address)?)
        .from_block(BlockNumberOrTag::Number(from_block_number))
        .to_block(BlockNumberOrTag::Number(to_block_number));

    let logs = provider.get_logs(&filter).await?;

    let mut tx = db_pool.begin().await?;
    for log in logs {
        let _ = EvmLogs::create(log, &mut *tx)
            .await
            .inspect_err(|error| eprintln!("Error saving log {error}"));
    }

    let _ = sync_log
        .update_last_synced_block_number(to_block_number, &mut *tx)
        .await
        .inspect_err(|error| eprintln!("Error updating last_synced_block_number {error}"));

    match tx.commit().await {
        Ok(_) => {
            println!("Saved logs for {address}, blocks: {from_block_number} to {to_block_number}",)
        }
        Err(err) => eprintln!("{err}"),
    }

    Ok(())
}
