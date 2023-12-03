use std::error::Error;
use std::sync::Arc;

use crate::domain::ports::ThreadSafe;
use crate::domain::{dto::user_dto::CreateUserDTO, entities};
use crate::repository::{user_repo, DbManager};

pub struct Context {
    manager: DbManager,
}

impl ThreadSafe for Context {}

impl Context {
    pub fn new(manager: DbManager) -> Arc<Self> {
        Arc::new(Self { manager })
    }
}

impl Context {
    pub async fn create_user(
        &self,
        payload: CreateUserDTO,
    ) -> Result<entities::User, Box<dyn Error + Send + Sync>> {
        let mut tx = self.manager.begin_tx().await?;

        let res = user_repo::create_user(&mut *tx, payload).await?;

        tx.commit().await?;
        Ok(res)
    }
}
