use std::fmt::{Debug, Display};

use indexer_db::entity::evm_logs::EvmLogsError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Missing `{0}` environment variable")]
    MissingEnvVar(String),

    #[error("Missing ABI file for contract: `{0}`")]
    MissingContractAbiFile(String),

    #[error("Missing ABI contents for contract: `{0}`")]
    InvalidAbi(String),

    #[error("Address: `{0}` is not supported")]
    UnsupportedAddress(String),

    #[error("Contract: `{0}` implementation not available in registry")]
    UnsupportedContract(String),

    #[error("Contract: `{0}` does not have event: `{1}`")]
    MissingEvent(String, String),

    #[error("Contract: `{0}` does not have event handler for: `{1}`")]
    MissingEventHandler(String, String),

    #[error("Invalid address: `{0}`")]
    InvalidAddress(String),

    #[error("EVM Logs error occurred")]
    EvmLogs {
        #[from]
        #[source]
        source: EvmLogsError,
    },
}
