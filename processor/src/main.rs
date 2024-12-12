use contracts::ContractRegistry;
use indexer_db::{entity::evm_logs::EvmLogs, initialize_database};
use processor::ContractHandler;
use std::error::Error;

mod contracts;
mod utils;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let contract_registry = ContractRegistry::new();

    let db_pool = initialize_database().await.unwrap();
    let unprocessed_logs = EvmLogs::find_all(&db_pool).await?;

    for log in unprocessed_logs {
        contract_registry
            .get_processor(log.address)
            .process(log)
            .await;
    }

    Ok(())
}
