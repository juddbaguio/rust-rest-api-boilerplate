use std::error::Error;
use std::sync::Arc;

use crate::domain::ports::ThreadSafe;
use crate::domain::{dto::user_dto::CreateUserDTO, entities};
use crate::repository::{user_repo, TransactionManager};

pub struct Context {
    manager: TransactionManager,
}

impl ThreadSafe for Context {}

impl Context {
    pub fn new(manager: TransactionManager) -> Arc<Self> {
        Arc::new(Self { manager })
    }
}

impl Context {
    pub async fn create_user(
        &self,
        payload: CreateUserDTO,
    ) -> Result<entities::User, Box<dyn Error + Send + Sync>> {
        let mut db_ctx = self.manager.init_db_context_with_tx().await?;

        let res = user_repo::create_user(&mut db_ctx, payload).await;
        if res.is_err() {
            db_ctx.rollback().await?;
        }

        db_ctx.commit().await?;
        Ok(res.unwrap())
    }
}
