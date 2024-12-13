use std::{collections::HashMap, env};

use contract_handler::ContractHandler;
use uniswap::UniswapV3Factory;

use crate::{error::AppError, utils};

pub mod contract_handler;
pub mod uniswap;

pub struct ContractRegistry {
    registry: HashMap<String, String>,
}

impl ContractRegistry {
    pub fn new() -> Result<Self, AppError> {
        // env format: NAME:address
        let contracts =
            env::var("CONTRACTS").map_err(|_| AppError::MissingEnvVar("CONTRACTS".to_string()))?;

        let mut contract_name_by_address: HashMap<String, String> = HashMap::new();
        contracts.split(",").for_each(|contract| {
            let contract_details: Vec<&str> = contract
                .split(":")
                .map(|address_str| address_str.trim())
                .collect();
            let (contract_name, contract_address) = (contract_details[0], contract_details[1]);
            contract_name_by_address
                .insert(contract_address.to_lowercase(), contract_name.to_string());
        });

        Ok(Self {
            registry: contract_name_by_address,
        })
    }

    pub fn get_processor(&self, address: [u8; 20]) -> Result<impl ContractHandler, AppError> {
        let log_address = utils::vec_to_hex(address.to_vec());
        let contract = self.registry.get(&log_address);

        if let Some(contract_name) = contract {
            match contract_name.as_str() {
                UniswapV3Factory::NAME => UniswapV3Factory::new(&log_address),
                unsupported => Err(AppError::UnsupportedContract(unsupported.into())),
            }
        } else {
            Err(AppError::UnsupportedAddress(log_address))
        }
    }
}
