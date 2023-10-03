use std::error::Error;

use futures::future::BoxFuture;
use sqlx::{PgConnection, Transaction};
use sqlx::{Pool, Postgres};

use crate::domain::ports::{ThreadSafe, UserRepo};

use self::user_repo::UserRepositoryInteractor;

pub mod models;
pub mod user_repo;

#[derive(Clone)]
pub struct TransactionManager {
    pg_pool: Pool<Postgres>,
}

pub struct TxCtx<'a> {
    #[allow(dead_code)]
    conn: &'a mut PgConnection,
}

impl ThreadSafe for TxCtx<'_> {}

impl TxCtx<'_> {
    #[allow(dead_code)]
    pub fn user_repo(&mut self) -> impl UserRepo + '_ {
        UserRepositoryInteractor { conn: self.conn }
    }
}

impl ThreadSafe for TransactionManager {}

impl TransactionManager {
    pub fn new(pg_pool: Pool<Postgres>) -> Self {
        Self { pg_pool }
    }

    #[allow(dead_code)]
    pub fn get_pool(&self) -> &Pool<Postgres> {
        &self.pg_pool
    }

    #[allow(dead_code)]
    pub async fn begin_tx(
        &self,
    ) -> Result<Transaction<'_, Postgres>, Box<dyn Error + Send + Sync>> {
        let tx = self.pg_pool.begin().await?;
        Ok(tx)
    }

    #[allow(dead_code)]
    pub async fn with_tx<F, B>(&self, exec_fn: F) -> Result<B, Box<dyn Error + Send + Sync>>
    where
        for<'a> F: FnOnce(&'a mut TxCtx) -> BoxFuture<'a, Result<B, Box<dyn Error + Send + Sync>>>,
        B: Send + Sync,
    {
        let mut tx = self.pg_pool.begin().await?;
        let mut tx_ctx = TxCtx { conn: &mut tx };
        let res = exec_fn(&mut tx_ctx).await?;
        tx.commit().await?;
        Ok(res)
    }
}
