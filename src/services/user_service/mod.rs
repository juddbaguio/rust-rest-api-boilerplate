use std::error::Error;
use std::sync::Arc;

use tokio::sync::mpsc::Sender;

use crate::domain::ports::ThreadSafe;
use crate::domain::{dto::user_dto::CreateUserDTO, entities};
use crate::repository::{user_repo, DbManager};

use crate::Event;

pub struct Context {
    manager: DbManager,
    pub tx: Sender<Event>,
}

impl ThreadSafe for Context {}

impl Context {
    pub fn new(manager: DbManager, tx: Sender<Event>) -> Arc<Self> {
        Arc::new(Self { manager, tx })
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
