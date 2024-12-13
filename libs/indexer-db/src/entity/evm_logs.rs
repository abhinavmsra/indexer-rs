use alloy::{
    primitives::{Address, Bytes, FixedBytes},
    rpc::types::Log,
};
use sqlx::{
    types::{chrono, BigDecimal},
    Executor, Postgres,
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum EvmLogsError {
    #[error("Failed to create a valid log data")]
    InvalidLogData,

    #[error("Invalid block number: `{0}`")]
    InvalidBlockNumber(String),
}

#[derive(sqlx::FromRow, Debug)]
pub struct EvmLogs {
    pub id: i32,
    pub block_number: BigDecimal,
    pub block_hash: [u8; 32],
    pub address: [u8; 20],
    pub transaction_hash: [u8; 32],
    pub data: Vec<u8>,
    pub event_signature: [u8; 32],
    pub topics: Vec<[u8; 32]>,
    pub transaction_index: i64,
    pub log_index: i64,
    pub removed: bool,
    pub created_at: chrono::NaiveDateTime,
}

impl TryInto<Log> for EvmLogs {
    type Error = EvmLogsError;

    fn try_into(self) -> Result<Log, Self::Error> {
        let transaction_hash = FixedBytes::<32>::from(self.transaction_hash);
        let contract_address = Address::from(self.address);
        let topics: Vec<FixedBytes<32>> = self.topics.iter().map(FixedBytes::<32>::from).collect();
        let data = Bytes::from(self.data);

        let inner = alloy::primitives::Log::new(contract_address, topics, data)
            .ok_or(EvmLogsError::InvalidLogData)?;

        let block_number = self
            .block_number
            .to_string()
            .parse::<u64>()
            .map_err(|_| EvmLogsError::InvalidBlockNumber(self.block_number.to_string()))?;

        Ok(Log {
            inner,
            block_number: Some(block_number),
            block_hash: None,
            block_timestamp: None,
            transaction_hash: Some(transaction_hash),
            transaction_index: None,
            log_index: None,
            removed: false,
        })
    }
}

impl EvmLogs {
    pub async fn create<'c, E>(log: Log, connection: E) -> Result<EvmLogs, sqlx::Error>
    where
        E: Executor<'c, Database = Postgres>,
    {
        let block_hash = log
            .block_hash
            .ok_or_else(|| sqlx::Error::Decode("Missing block hash".into()))?
            .to_vec();

        let block_number: BigDecimal = log
            .block_number
            .ok_or_else(|| sqlx::Error::Decode("Missing block number".into()))?
            .try_into()
            .map_err(|_| sqlx::Error::Decode("Block number exceeds BigDecimal range".into()))?;

        let transaction_index: i64 = log
            .transaction_index
            .ok_or_else(|| sqlx::Error::Decode("Missing transaction index".into()))?
            .try_into()
            .map_err(|_| sqlx::Error::Decode("Transaction index exceeds i64 range".into()))?;

        let log_index: i64 = log
            .log_index
            .ok_or_else(|| sqlx::Error::Decode("Missing log index".into()))?
            .try_into()
            .map_err(|_| sqlx::Error::Decode("Log index exceeds i64 range".into()))?;

        let transaction_hash = log
            .transaction_hash
            .ok_or_else(|| sqlx::Error::Decode("Missing transaction hash".into()))?
            .to_vec();

        let address = log.address().to_vec();

        let event_signature: &[u8] = log.topics()[0].as_slice();

        let topics: Vec<&[u8]> = log
            .topics()
            .iter()
            .map(|topic| -> &[u8] { topic.as_slice() })
            .collect();

        let log_data: Vec<u8> = log.inner.data.data.to_vec();

        // Insert log into the database and return the inserted row
        let query = r#"
            INSERT INTO evm_logs (block_hash, block_number, address, transaction_hash, transaction_index, event_signature, topics, data, log_index, removed)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            RETURNING *
        "#;

        sqlx::query_as::<_, EvmLogs>(query)
            .bind(block_hash)
            .bind(block_number)
            .bind(address)
            .bind(transaction_hash)
            .bind(transaction_index)
            .bind(event_signature)
            .bind(topics)
            .bind(log_data)
            .bind(log_index)
            .bind(log.removed)
            .fetch_one(connection)
            .await
    }

    pub async fn find_all<'c, E>(connection: E) -> Result<Vec<EvmLogs>, sqlx::Error>
    where
        E: Executor<'c, Database = Postgres>,
    {
        sqlx::query_as::<_, EvmLogs>("SELECT * FROM evm_logs")
            .fetch_all(connection)
            .await
    }
}
