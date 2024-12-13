use contracts::contract_handler::ContractHandler;
use contracts::ContractRegistry;
use indexer_db::{entity::evm_logs::EvmLogs, initialize_database};
use std::error::Error;

mod contracts;
mod error;
mod utils;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let contract_registry = ContractRegistry::new()?;

    let db_pool = initialize_database().await?;
    let unprocessed_logs = EvmLogs::find_all(&db_pool).await?;

    for log in unprocessed_logs {
        let processor = contract_registry.get_processor(log.address);
        match processor {
            Ok(processor_handle) => {
                if let Err(error) = processor_handle.process(log).await {
                    eprintln!("{}", error);
                }
            }
            Err(error) => eprintln!("{}", error),
        }
    }

    Ok(())
}
