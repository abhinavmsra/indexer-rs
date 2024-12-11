use alloy::rpc::types::Log;
use sqlx::{
    types::{chrono, BigDecimal},
    Executor, Postgres,
};

#[derive(sqlx::FromRow, Debug)]
pub struct EvmLogs {
    pub id: i32,
    pub block_number: BigDecimal,
    pub address: [u8; 20],
    pub transaction_hash: [u8; 32],
    pub data: Vec<u8>,
    pub event_signature: [u8; 32],
    pub topics: Vec<[u8; 32]>,
    pub created_at: chrono::NaiveDateTime,
}

impl EvmLogs {
    pub async fn create<'c, E>(log: Log, connection: E) -> Result<EvmLogs, sqlx::Error>
    where
        E: Executor<'c, Database = Postgres>,
    {
        let block_number: BigDecimal = log
            .block_number
            .ok_or_else(|| sqlx::Error::Decode("Missing block number".into()))?
            .try_into()
            .map_err(|_| sqlx::Error::Decode("Block number exceeds BigDecimal range".into()))?;

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
            INSERT INTO evm_logs (block_number, address, transaction_hash, event_signature, topics, data)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING *
        "#;

        sqlx::query_as::<_, EvmLogs>(query)
            .bind(block_number)
            .bind(address)
            .bind(transaction_hash)
            .bind(event_signature)
            .bind(topics)
            .bind(log_data)
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
