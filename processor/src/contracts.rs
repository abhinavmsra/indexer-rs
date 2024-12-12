use std::{collections::HashMap, env};

use processor::ContractHandler;
use uniswap::UniswapV3Factory;

use crate::utils;

pub mod uniswap;

pub struct ContractRegistry {
    registry: HashMap<String, String>,
}

impl ContractRegistry {
    pub fn new() -> Self {
        // env format: NAME:address
        let contracts = env::var("CONTRACTS").expect("Missing CONTRACTS environment variable");
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

        Self {
            registry: contract_name_by_address,
        }
    }

    pub fn get_processor(&self, address: [u8; 20]) -> impl ContractHandler {
        let log_address = utils::vec_to_hex(address.to_vec());
        let contract = self.registry.get(&log_address).unwrap().as_str();

        match contract {
            UniswapV3Factory::NAME => UniswapV3Factory::new(&log_address),
            unsupported => panic!("Unsupported contract: {}", unsupported),
        }
    }
}
