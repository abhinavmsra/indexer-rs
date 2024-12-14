use contracts::contract_handler::ContractHandler;
use indexer_db::{entity::evm_logs::EvmLogs, initialize_database};
use service::process_logs;
use std::{env, error::Error};
use tokio::time::{sleep, Duration};

mod contracts;
mod error;
mod service;
mod utils;

mod defaults {
    pub const POLL_INTERVAL: &str = "10";
    pub const BATCH_SIZE: &str = "25";
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let db_pool = initialize_database().await?;
    let poll_interval = env::var("POLL_INTERVAL")
        .or::<String>(Ok(defaults::POLL_INTERVAL.into()))?
        .parse::<u64>()?;

    let sleep_duration = Duration::from_secs(poll_interval);

    loop {
        let unprocessed_count = match EvmLogs::count(&db_pool).await {
            Ok(count) => count,
            Err(err) => {
                eprintln!(
                    "Error counting unprocessed logs: {err}. Sleeping for {} seconds...",
                    sleep_duration.as_secs()
                );

                sleep(sleep_duration).await;
                continue;
            }
        };

        match unprocessed_count {
            Some(count) => {
                println!("Found {count} unprocessed logs. Starting processing...",);

                if let Err(err) = process_logs(&db_pool).await {
                    eprintln!("Error processing logs: {err}");
                }
            }
            None => {
                println!(
                    "No unprocessed logs found. Sleeping for {} seconds...",
                    sleep_duration.as_secs()
                );
                sleep(sleep_duration).await;
            }
        }
    }
}
