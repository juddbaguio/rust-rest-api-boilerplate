use std::error::Error;

use crate::repository::TransactionManager;

use super::{dto::user_dto::CreateUserDTO, entities};
use async_trait::async_trait;

pub trait ThreadSafe: Send + Sync {}

#[async_trait]
pub trait UserRepo {
    async fn create_user(
        &mut self,
        db: &mut TransactionManager,
        payload: CreateUserDTO,
    ) -> Result<entities::User, Box<dyn Error + Send + Sync>>;
}
