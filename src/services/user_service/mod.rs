use std::error::Error;
use std::sync::Arc;

use futures::future::FutureExt;

use crate::domain::ports::{ThreadSafe, UserRepo};
use crate::domain::{dto::user_dto::CreateUserDTO, entities};
use crate::repository::TransactionManager;

#[derive(Clone)]
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
        self.manager
            .with_tx(|ctx| {
                async {
                    let mut ur = ctx.user_repo();

                    ur.create_user(payload).await
                }
                .boxed()
            })
            .await
    }
}
