use alloy::{json_abi::JsonAbi, primitives::Address, rpc::types::Log};
use processor::ContractHandler;

pub struct UniswapV3Factory {
    #[allow(dead_code)]
    pub address: Address,
    pub abi: JsonAbi,
}

impl ContractHandler for UniswapV3Factory {
    const NAME: &str = "uniswap_v3_factory";

    fn new(address: &str) -> Self {
        let contract_address = address.parse::<Address>().unwrap();

        Self {
            address: contract_address,
            abi: Self::get_abi_for_contract(Self::NAME),
        }
    }

    fn abi(&self) -> &JsonAbi {
        &self.abi
    }

    // Map event to handlers here
    async fn handle_event(&self, event: &str, log: &Log) {
        match event {
            "PoolCreated" => self.pool_created_handler(log).await,
            "OwnerChanged" => self.owner_changed_handler(log).await,
            "FeeAmountEnabled" => self.fee_amount_enabled_handler(log).await,
            unsupported => eprintln!("Unsupported event, {}", unsupported),
        }
    }
}

// Implement Handlers here
impl UniswapV3Factory {
    async fn pool_created_handler(&self, _log: &Log) {
        println!("pool_created_handler called");
    }

    async fn owner_changed_handler(&self, _log: &Log) {
        println!("owner_changed_handler called");
    }

    async fn fee_amount_enabled_handler(&self, _log: &Log) {
        println!("owner_changed_handler called");
    }
}
