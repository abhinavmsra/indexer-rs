use std::{collections::HashMap, env, error::Error};

use alloy::rpc::types::Log;
use alloy::{
    json_abi::JsonAbi,
    primitives::{keccak256, Address, Bytes, FixedBytes},
};
use contracts::event_factory;
use indexer_db::{entity::evm_logs::EvmLogs, initialize_database};
use inflector::Inflector;

mod contracts;

mod utils {
    pub fn vec_to_hex<T>(vec: Vec<T>) -> String
    where
        T: std::fmt::LowerHex + Copy,
    {
        vec.iter().fold(String::new(), |mut acc, val| {
            acc.push_str(&format!("{:02x}", val));
            acc
        })
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let app_events = event_factory();
    // env format: NAME:address
    let contracts = env::var("CONTRACTS").expect("Missing CONTRACTS environment variable");
    let mut contract_address_to_abi = HashMap::new();
    contracts.split(",").for_each(|contract| {
        let contract_details: Vec<&str> = contract
            .split(":")
            .map(|address_str| address_str.trim())
            .collect();
        let (contract_name, contract_address) = (contract_details[0], contract_details[1]);
        let contract_abi =
            std::fs::read(format!("processor/artifacts/abi/{}.json", contract_name)).unwrap();
        contract_address_to_abi.insert(
            contract_address.to_lowercase(),
            (contract_name, contract_abi),
        );
    });

    let db_pool = initialize_database().await.unwrap();
    let unprocessed_logs = EvmLogs::find_all(&db_pool).await?;

    for log in unprocessed_logs {
        let log_address = utils::vec_to_hex(log.address.to_vec());
        let log_event_signature = format!("0x{}", utils::vec_to_hex(log.event_signature.to_vec()));
        let (contract_name, contract_abi) = contract_address_to_abi.get(&log_address).unwrap();
        let contract_abi: JsonAbi = serde_json::from_slice(contract_abi)?;
        let (event_name, _params) = contract_abi
            .events
            .iter()
            .find(|(_name, event_params)| {
                let signature = event_params[0].signature();
                let signature_hash = keccak256(signature).to_string();

                log_event_signature == signature_hash
            })
            .expect("Invalid event");

        let fn_name = format!(
            "contracts::{}::{}_handler",
            contract_name,
            event_name.to_snake_case()
        );
        let function = app_events.get(fn_name.as_str());

        if let Some(handler) = function {
            let topics: Vec<FixedBytes<32>> =
                log.topics.iter().map(FixedBytes::<32>::from).collect();

            let data = Bytes::from(log.data);
            let contract_address = Address::from(log.address);
            let block_number = log.block_number.to_string().parse::<u64>().unwrap();
            let transaction_hash = FixedBytes::<32>::from(log.transaction_hash);
            let log_data = alloy::primitives::Log::new(contract_address, topics, data).unwrap();

            let log = Log {
                inner: log_data,
                block_number: Some(block_number),
                block_hash: None,
                block_timestamp: None,
                transaction_hash: Some(transaction_hash),
                transaction_index: None,
                log_index: None,
                removed: false,
            };

            handler(&log);
        }
    }

    Ok(())
}
