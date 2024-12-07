use sqlx::{types::chrono, Executor, Postgres};

#[derive(sqlx::FromRow, Debug)]
pub struct EvmChains {
    pub id: i64,
    pub name: String,
    pub last_synced_block_number: i64,
    pub block_time: i32,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

impl EvmChains {
    pub async fn fetch_by_id<'c, E>(id: u64, connection: E) -> Result<EvmChains, sqlx::Error>
    where
        E: Executor<'c, Database = Postgres>,
    {
        let query = "SELECT * FROM evm_chains WHERE id = $1";

        sqlx::query_as::<_, EvmChains>(query)
            .bind(id as i64)
            .fetch_one(connection)
            .await
    }

    pub async fn update_last_synced_block_number<'c, E>(
        &self,
        block_number: u64,
        connection: E,
    ) -> Result<EvmChains, sqlx::Error>
    where
        E: Executor<'c, Database = Postgres>,
    {
        let query = "UPDATE evm_chains SET last_synced_block_number = $1 WHERE id = $2 RETURNING *";

        sqlx::query_as::<_, EvmChains>(query)
            .bind(block_number as i64)
            .bind(self.id)
            .fetch_one(connection)
            .await
    }
}
