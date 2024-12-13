use std::env;

use alloy::{json_abi::JsonAbi, primitives::keccak256, rpc::types::Log};
use indexer_db::entity::evm_logs::EvmLogs;

use crate::{error::AppError, utils};

pub trait ContractHandler {
    const NAME: &str;

    fn get_abi_for_contract(contract_name: &str) -> Result<JsonAbi, AppError> {
        let artifacts_base_path = env::var("ARTIFACTS_BASE_PATH")
            .map_err(|_| AppError::MissingEnvVar("ARTIFACTS_BASE_PATH".into()))?;

        let abi_file_path = format!("{}/{}.json", artifacts_base_path, contract_name);

        let abi_contents = std::fs::read(abi_file_path)
            .map_err(|_| AppError::MissingContractAbiFile(contract_name.into()))?;

        let abi = serde_json::from_slice::<JsonAbi>(&abi_contents)
            .map_err(|_| AppError::InvalidAbi(contract_name.into()))?;

        Ok(abi)
    }

    fn event_signature_to_name(&self, signature: [u8; 32]) -> Result<String, AppError> {
        let log_event_signature = format!("0x{}", utils::vec_to_hex(signature.to_vec()));
        let event = self.abi().events.iter().find(|(_name, event_params)| {
            let signature = event_params[0].signature();
            let signature_hash = keccak256(signature).to_string();

            log_event_signature == signature_hash
        });

        if let Some((event_name, _params)) = event {
            Ok(event_name.to_string())
        } else {
            Err(AppError::MissingEvent(
                Self::NAME.into(),
                log_event_signature,
            ))
        }
    }

    async fn process(&self, unprocessed_log: EvmLogs) -> Result<(), AppError> {
        let event_name = self.event_signature_to_name(unprocessed_log.event_signature)?;
        let log: Log = unprocessed_log.try_into()?;
        self.handle_event(&event_name, &log).await?;

        Ok(())
    }

    fn new(address: &str) -> Result<Self, AppError>
    where
        Self: Sized;
    async fn handle_event(&self, event: &str, log: &Log) -> Result<(), AppError>;
    fn abi(&self) -> &JsonAbi;
}
