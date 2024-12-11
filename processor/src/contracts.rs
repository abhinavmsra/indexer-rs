use alloy::rpc::types::Log;
use std::collections::HashMap;

pub mod uniswap;

pub fn event_factory() -> HashMap<&'static str, fn(&Log)> {
    HashMap::from([(
        "contracts::uniswap_v3_factory::pool_created_handler",
        uniswap::pool_created_handler as fn(&Log),
    )])
}
