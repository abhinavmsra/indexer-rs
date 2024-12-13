use std::{
    error::Error,
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

use alloy::{
    eips::BlockNumberOrTag,
    primitives::Address,
    providers::{Provider, ProviderBuilder},
    rpc::types::Filter,
};
use indexer_db::entity::{evm_chains::EvmChains, evm_logs::EvmLogs};
use sqlx::{Pool, Postgres};
use tower::Service;

pub struct ListenerService {
    pub chain_id: u64,
    pub rpc_url: String,
    pub addresses: Vec<Address>,
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
        let rpc_url = self.rpc_url.clone();
        let addresses = self.addresses.clone();

        Box::pin(async move { fetch_and_save_logs(chain_id, rpc_url, db_pool, addresses).await })
    }
}

pub async fn fetch_and_save_logs(
    chain_id: u64,
    rpc_url: String,
    db_pool: Pool<Postgres>,
    addresses: Vec<Address>,
) -> Result<(), Box<dyn Error>> {
    let provider = ProviderBuilder::new().on_builtin(&rpc_url).await?;
    let evm_chain = EvmChains::fetch_by_id(chain_id, &db_pool).await?;
    let latest_block = provider.get_block_number().await?;
    if latest_block == evm_chain.last_synced_block_number as u64 {
        println!("Fully indexed");
        return Ok(());
    }

    let from_block_number = match evm_chain.last_synced_block_number as u64 {
        0 => 0,
        block_number => block_number + 1_u64,
    };

    let to_block_number = match evm_chain.last_synced_block_number as u64 {
        0 => latest_block,
        block_number => std::cmp::min(block_number + 25_u64, latest_block),
    };

    let filter = Filter::new()
        .address(addresses)
        .from_block(BlockNumberOrTag::Number(from_block_number))
        .to_block(BlockNumberOrTag::Number(to_block_number));

    let logs = provider.get_logs(&filter).await?;

    let mut tx = db_pool.begin().await?;
    for log in logs {
        let _ = EvmLogs::create(log, &mut *tx)
            .await
            .inspect_err(|error| eprintln!("Error saving log {error}"));
    }

    let _ = evm_chain
        .update_last_synced_block_number(to_block_number, &mut *tx)
        .await
        .inspect_err(|error| eprintln!("Error updating last_synced_block_number {error}"));

    match tx.commit().await {
        Ok(_) => println!("Saved logs for blocks: {from_block_number} to {to_block_number}",),
        Err(err) => eprintln!("{err}"),
    }

    Ok(())
}
