use std::fmt::Debug;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Missing `{0}` environment variable")]
    MissingEnvVar(String),

    #[error("Invalid ChainID: `{0}`")]
    InvalidChainID(String),
}
