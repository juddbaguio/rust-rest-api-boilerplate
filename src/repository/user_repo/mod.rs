// use async_trait::async_trait;

use std::error::Error;

use sqlx::{Executor, Postgres};

use crate::domain::{dto::user_dto::CreateUserDTO, entities};

use super::models::users;

pub async fn create_user<'e, 'c: 'e, E>(
    db_executor: E,
    payload: CreateUserDTO,
) -> Result<entities::User, Box<dyn Error + Send + Sync>>
where
    E: 'e + Executor<'c, Database = Postgres>,
{
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

    let create_user_res = create_user_stmt.fetch_one(db_executor).await?;

    Ok(entities::User::from(create_user_res))
}
