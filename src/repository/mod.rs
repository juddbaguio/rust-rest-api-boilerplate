use std::error::Error;

use sqlx::Transaction;
use sqlx::{Pool, Postgres};

use crate::domain::ports::ThreadSafe;

pub mod models;
pub mod user_repo;

pub struct DbManager {
    pg_pool: Pool<Postgres>,
}

impl ThreadSafe for DbManager {}

impl DbManager {
    pub fn new(pg_pool: Pool<Postgres>) -> Self {
        Self { pg_pool }
    }

    pub async fn begin_tx(
        &self,
    ) -> Result<Transaction<'_, Postgres>, Box<dyn Error + Send + Sync>> {
        let tx = self.pg_pool.begin().await?;
        Ok(tx)
    }
}
