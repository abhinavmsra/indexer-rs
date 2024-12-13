use std::{env, str::FromStr, time::Duration};

use alloy::primitives::Address;
use error::AppError;
use indexer_db::{entity::evm_chains::EvmChains, initialize_database};
use service::ListenerService;
use tower::{Service, ServiceBuilder, ServiceExt};

mod error;
mod service;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let rpc_url = env::var("RPC_URL").map_err(|_| AppError::MissingEnvVar("RPC_URL".into()))?;

    let contract_addresses = env::var("CONTRACT_ADDRESSES")
        .map_err(|_| AppError::MissingEnvVar("CONTRACT_ADDRESSES".into()))?;

    let chain_id_env =
        env::var("CHAIN_ID").map_err(|_| AppError::MissingEnvVar("CHAIN_ID".into()))?;

    let chain_id = chain_id_env
        .parse::<u64>()
        .map_err(|_| AppError::InvalidChainID(chain_id_env))?;

    let addresses: Vec<Address> = contract_addresses
        .split(",")
        .map(|address_str| Address::from_str(address_str.trim()).unwrap())
        .collect();

    let db_pool = initialize_database().await.unwrap();

    let evm_chain = EvmChains::fetch_by_id(chain_id, &db_pool).await?;

    let mut service = ServiceBuilder::new()
        .rate_limit(1, Duration::from_secs(evm_chain.block_time as u64))
        .service(ListenerService {
            chain_id,
            db_pool,
            rpc_url,
            addresses,
        });

    loop {
        if service.ready().await.is_ok() {
            match service.call(()).await {
                Ok(()) => {}
                Err(err) => {
                    eprintln!("Failed to indexed: {:?}", err);
                }
            }
        }
    }
}
