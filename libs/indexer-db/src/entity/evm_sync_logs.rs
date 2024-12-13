use sqlx::{types::chrono, Executor, Postgres};

#[derive(sqlx::FromRow, Debug)]
pub struct EvmSyncLogs {
    pub address: [u8; 20],
    pub last_synced_block_number: i64,

    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

impl EvmSyncLogs {
    pub async fn find_all<'c, E>(connection: E) -> Result<Vec<EvmSyncLogs>, sqlx::Error>
    where
        E: Executor<'c, Database = Postgres>,
    {
        sqlx::query_as::<_, EvmSyncLogs>("SELECT * FROM evm_sync_logs")
            .fetch_all(connection)
            .await
    }

    pub async fn find_by_address<'c, E>(
        address: &str,
        connection: E,
    ) -> Result<Option<EvmSyncLogs>, sqlx::Error>
    where
        E: Executor<'c, Database = Postgres>,
    {
        let query = "SELECT * FROM evm_sync_logs WHERE address = $1::BYTEA";

        sqlx::query_as::<_, EvmSyncLogs>(query)
            .bind(format!("\\x{address}"))
            .fetch_optional(connection)
            .await
    }

    pub async fn create<'c, E>(
        address: &str,
        chain_id: u64,
        last_synced_block_number: Option<i64>,
        connection: E,
    ) -> Result<EvmSyncLogs, sqlx::Error>
    where
        E: Executor<'c, Database = Postgres>,
    {
        let query = r#"
          INSERT INTO evm_sync_logs (address, chain_id, last_synced_block_number)
          VALUES ($1::BYTEA, $2, $3)
          RETURNING *
        "#;

        sqlx::query_as::<_, EvmSyncLogs>(query)
            .bind(format!("\\x{address}"))
            .bind(chain_id as i64)
            .bind(last_synced_block_number.or(Some(0)))
            .fetch_one(connection)
            .await
    }

    pub async fn find_or_create_by_address<'c, E>(
        address: &str,
        chain_id: u64,
        connection: E,
    ) -> Result<EvmSyncLogs, sqlx::error::Error>
    where
        E: Executor<'c, Database = Postgres> + Clone,
    {
        let record = Self::find_by_address(address, connection.clone()).await?;
        if let Some(log_record) = record {
            return Ok(log_record);
        }

        let new_record = Self::create(address, chain_id, None, connection.clone()).await?;
        Ok(new_record)
    }

    pub async fn update_last_synced_block_number<'c, E>(
        &self,
        block_number: u64,
        connection: E,
    ) -> Result<EvmSyncLogs, sqlx::Error>
    where
        E: Executor<'c, Database = Postgres>,
    {
        let query =
            "UPDATE evm_sync_logs SET last_synced_block_number = $1 WHERE address = $2 RETURNING *";

        sqlx::query_as::<_, EvmSyncLogs>(query)
            .bind(block_number as i64)
            .bind(self.address)
            .fetch_one(connection)
            .await
    }
}
