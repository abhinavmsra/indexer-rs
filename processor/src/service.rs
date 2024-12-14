use indexer_db::entity::evm_logs::EvmLogs;
use sqlx::{Pool, Postgres};
use std::{env, error::Error};
use tokio::task::JoinSet;

use crate::{contracts::ContractRegistry, defaults, ContractHandler};

pub async fn process_logs(db_pool: &Pool<Postgres>) -> Result<(), Box<dyn Error>> {
    let contract_registry = ContractRegistry::new()?;
    let batch_size = env::var("BATCH_SIZE")
        .or::<String>(Ok(defaults::BATCH_SIZE.into()))?
        .parse::<i32>()?;

    let unprocessed_logs = EvmLogs::find_all(batch_size, db_pool).await?;

    let mut futures = JoinSet::new();
    for log in unprocessed_logs {
        let processor_result = contract_registry.get_processor(log.address);

        match processor_result {
            Ok(processor) => {
                let service_db_pool = db_pool.clone();
                let log_id = log.id;

                futures.spawn(async move {
                    match processor.process(log).await {
                        Ok(_) => {
                            if let Err(error) = EvmLogs::delete(log_id, &service_db_pool).await {
                                eprintln!("{}", error)
                            }
                        }
                        Err(error) => eprintln!("{}", error),
                    }
                });
            }
            Err(error) => eprintln!("{}", error),
        }
    }

    futures.join_all().await;

    Ok(())
}
