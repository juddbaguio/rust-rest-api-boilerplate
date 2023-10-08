// use async_trait::async_trait;

use std::error::Error;

use crate::domain::{dto::user_dto::CreateUserDTO, entities};

use super::{models::users, DbContext};

// pub struct UserRepositoryInteractor {}

// impl ThreadSafe for UserRepositoryInteractor {}

// #[async_trait]
// impl UserRepo for UserRepositoryInteractor {}

pub async fn create_user<'a>(
    db: &mut DbContext<'a>,
    payload: CreateUserDTO,
) -> Result<entities::User, Box<dyn Error + Send + Sync>> {
    let create_user_stmt = sqlx::query_as::<_, users::Model>(
        r#"
            INSERT INTO users (
                first_name,
                middle_name,
                last_name,
                username,
                password
            )
            values(
                $1,
                $2,
                $3,
                $4,
                $5
            )
            RETURNING *
        "#,
    )
    .bind(payload.first_name)
    .bind(payload.middle_name)
    .bind(payload.last_name)
    .bind(payload.username)
    .bind(payload.password);

    let create_user_res = match db.pg_tx {
        Some(ref mut tx) => create_user_stmt.fetch_one(&mut **tx).await?,
        None => create_user_stmt.fetch_one(&db.pg_pool).await?,
    };

    Ok(entities::User::from(create_user_res))
}
