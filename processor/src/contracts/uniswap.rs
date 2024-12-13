use alloy::{json_abi::JsonAbi, primitives::Address, rpc::types::Log};

use crate::error::AppError;

use super::contract_handler::ContractHandler;

pub struct UniswapV3Factory {
    #[allow(dead_code)]
    pub address: Address,
    pub abi: JsonAbi,
}

impl ContractHandler for UniswapV3Factory {
    const NAME: &str = "uniswap_v3_factory";

    fn new(address: &str) -> Result<Self, AppError> {
        let contract_address = address
            .parse::<Address>()
            .map_err(|_| AppError::InvalidAddress(address.into()))?;

        let contract_abi = Self::get_abi_for_contract(Self::NAME)?;

        Ok(Self {
            address: contract_address,
            abi: contract_abi,
        })
    }

    fn abi(&self) -> &JsonAbi {
        &self.abi
    }

    // Map event to handlers here
    async fn handle_event(&self, event: &str, log: &Log) -> Result<(), AppError> {
        match event {
            "PoolCreated" => self.pool_created_handler(log).await,
            "OwnerChanged" => self.owner_changed_handler(log).await,
            "FeeAmountEnabled" => self.fee_amount_enabled_handler(log).await,
            unsupported => Err(AppError::MissingEventHandler(
                Self::NAME.into(),
                unsupported.into(),
            )),
        }
    }
}

// Implement Handlers here
impl UniswapV3Factory {
    async fn pool_created_handler(&self, _log: &Log) -> Result<(), AppError> {
        println!("pool_created_handler called");
        Ok(())
    }

    async fn owner_changed_handler(&self, _log: &Log) -> Result<(), AppError> {
        println!("owner_changed_handler called");
        Ok(())
    }

    async fn fee_amount_enabled_handler(&self, _log: &Log) -> Result<(), AppError> {
        println!("owner_changed_handler called");
        Ok(())
    }
}
