use alloy::rpc::types::Log;

pub fn pool_created_handler(log: &Log) {
    println!("pool_created_handler called with log: {:#?}", log);
}
