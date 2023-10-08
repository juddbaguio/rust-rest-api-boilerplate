use std::error::Error;

use sqlx::Transaction;
use sqlx::{Pool, Postgres};

use crate::domain::ports::ThreadSafe;

pub mod models;
pub mod user_repo;

pub struct TransactionManager {
    pg_pool: Pool<Postgres>,
}

pub struct DbContext<'a> {
    pg_pool: Pool<Postgres>,
    pg_tx: Option<Transaction<'a, Postgres>>,
}

impl ThreadSafe for TransactionManager {}

impl TransactionManager {
    pub fn new(pg_pool: Pool<Postgres>) -> Self {
        Self { pg_pool }
    }

    #[allow(dead_code)]
    pub fn init_db_context(&self) -> DbContext {
        DbContext {
            pg_pool: self.pg_pool.clone(),
            pg_tx: None,
        }
    }

    pub async fn init_db_context_with_tx(&self) -> Result<DbContext, Box<dyn Error + Send + Sync>> {
        let tx = self.pg_pool.begin().await?;

        Ok(DbContext {
            pg_pool: self.pg_pool.clone(),
            pg_tx: Some(tx),
        })
    }
}

impl<'a> DbContext<'a> {
    #[allow(dead_code)]
    pub async fn begin_tx(&mut self) -> Result<(), Box<dyn Error + Send + Sync>> {
        let tx = self.pg_pool.begin().await?;
        self.pg_tx = Some(tx);

        Ok(())
    }

    #[allow(dead_code)]
    pub async fn commit(&mut self) -> Result<(), Box<dyn Error + Send + Sync>> {
        if let Some(tx) = self.pg_tx.take() {
            tx.commit().await?;
        }

        Ok(())
    }

    #[allow(dead_code)]
    pub async fn rollback(&mut self) -> Result<(), Box<dyn Error + Send + Sync>> {
        if let Some(tx) = self.pg_tx.take() {
            tx.rollback().await?;
        }

        Ok(())
    }
}
