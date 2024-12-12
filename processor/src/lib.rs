use std::env;

use alloy::{
    json_abi::JsonAbi,
    primitives::{keccak256, Address, Bytes, FixedBytes},
    rpc::types::Log,
};
use indexer_db::entity::evm_logs::EvmLogs;

mod utils;

pub trait ContractHandler {
    const NAME: &str;

    fn get_abi_for_contract(contract_name: &str) -> JsonAbi {
        let artifacts_base_path = env::var("ARTIFACTS_BASE_PATH")
            .expect("Missing ARTIFACTS_BASE_PATH environment variable");
        let abi_file_path = format!("{}/{}.json", artifacts_base_path, contract_name);

        let abi_contents = std::fs::read(abi_file_path).unwrap();
        serde_json::from_slice::<JsonAbi>(&abi_contents).unwrap()
    }

    fn event_signature_to_name(&self, signature: [u8; 32]) -> String {
        let log_event_signature = format!("0x{}", utils::vec_to_hex(signature.to_vec()));
        let (event_name, _params) = self
            .abi()
            .events
            .iter()
            .find(|(_name, event_params)| {
                let signature = event_params[0].signature();
                let signature_hash = keccak256(signature).to_string();

                log_event_signature == signature_hash
            })
            .expect("Invalid event");

        event_name.to_string()
    }

    fn process(&self, unprocessed_log: EvmLogs) -> impl std::future::Future<Output = ()> + Send
    where
        Self: Sync,
    {
        async move {
            let event_name = self.event_signature_to_name(unprocessed_log.event_signature);

            let topics: Vec<FixedBytes<32>> = unprocessed_log
                .topics
                .iter()
                .map(FixedBytes::<32>::from)
                .collect();

            let data = Bytes::from(unprocessed_log.data);
            let contract_address = Address::from(unprocessed_log.address);
            let block_number = unprocessed_log
                .block_number
                .to_string()
                .parse::<u64>()
                .unwrap();
            let transaction_hash = FixedBytes::<32>::from(unprocessed_log.transaction_hash);

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

            self.handle_event(&event_name, &log).await;
        }
    }

    fn new(address: &str) -> Self;
    fn handle_event(&self, event: &str, log: &Log) -> impl std::future::Future<Output = ()> + Send;
    fn abi(&self) -> &JsonAbi;
}
