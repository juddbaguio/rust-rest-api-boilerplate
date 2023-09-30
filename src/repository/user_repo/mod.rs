use async_trait::async_trait;
use sqlx::PgConnection;

use std::error::Error;

use crate::domain::{
    dto::user_dto::CreateUserDTO,
    entities,
    ports::{ThreadSafe, UserRepo},
};

use super::models::users;

// #[allow(dead_code)]
// pub async fn create_user<'a, E>(
//     executor: E,
//     payload: CreateUserDTO,
// ) -> Result<entities::User, Box<dyn Error + Send + Sync>>
// where
//     E: 'a + Executor<'a, Database = Postgres>,
// {
//     let create_user_stmt = sqlx::query_as::<_, users::Model>(
//         r#"
//             INSERT INTO users (
//                 first_name,
//                 middle_name,
//                 last_name,
//                 username,
//                 password
//             )
//             values(
//                 $1,
//                 $2,
//                 $3,
//                 $4,
//                 $5
//             )
//             RETURNING *
//         "#,
//     )
//     .bind(payload.first_name)
//     .bind(payload.middle_name)
//     .bind(payload.last_name)
//     .bind(payload.username)
//     .bind(payload.password);

//     let create_user_res = create_user_stmt.fetch_one(executor).await?;

//     Ok(entities::User::from(create_user_res))
// }

pub struct UserRepositoryInteractor<'a> {
    #[allow(dead_code)]
    pub conn: &'a mut PgConnection,
}

impl ThreadSafe for UserRepositoryInteractor<'_> {}

#[async_trait]
impl UserRepo for UserRepositoryInteractor<'_> {
    async fn create_user(
        &mut self,
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

        let create_user_res = create_user_stmt.fetch_one(&mut *self.conn).await?;

        Ok(entities::User::from(create_user_res))
    }
}
