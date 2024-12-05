use std::{env, str::FromStr, time::Duration};

use alloy::primitives::Address;
use indexer_db::{entity::evm_chains::EvmChains, initialize_database};
use service::ListenerService;
use tower::{Service, ServiceBuilder, ServiceExt};

mod service;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let rpc_url = env::var("RPC_URL").expect("Missing RPC_URL environment variable");
    let chain_id = env::var("CHAIN_ID")
        .expect("Missing CHAIN_ID environment variable")
        .parse::<u64>()
        .expect("Invalid chain id");

    let contract_addresses =
        env::var("CONTRACT_ADDRESSES").expect("Missing RPC_URL environment variable");

    let addresses_to_listen = contract_addresses.split(",").map(|address_str| {
        let address = address_str.trim();
        Address::from_str(address).unwrap_or_else(|_| panic!("Invalid address: {address_str:?}"))
    });

    let db_pool = initialize_database().await.unwrap();

    let evm_chain = EvmChains::fetch_by_id(chain_id, &db_pool).await?;

    let mut service = ServiceBuilder::new()
        .rate_limit(1, Duration::from_secs(evm_chain.block_time as u64))
        .service(ListenerService {
            chain_id,
            db_pool,
            rpc_url,
            addresses: addresses_to_listen.collect(),
        });

    loop {
        if service.ready().await.is_ok() {
            match service.call(()).await {
                Ok(()) => {
                    println!("Indexed block");
                }
                Err(err) => {
                    eprintln!("Failed to indexed: {:?}", err);
                }
            }
        }
    }
}
