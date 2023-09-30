use std::error::Error;

use tracing::info;

use crate::{
    api::users::initialize_users_api, repository::TransactionManager, services::user_service,
};

mod api;
mod domain;
mod infrastructure;
mod repository;
mod services;

pub async fn run() -> Result<(), Box<dyn Error>> {
    let db_conn = infrastructure::database::connect().await?;
    let manager = TransactionManager::new(db_conn.clone());

    let user_ctx = user_service::Context::new(manager);
    let users_api = initialize_users_api(user_ctx);

    info!("API starting @ port http://127.0.0.1:3000");
    axum::Server::bind(&"127.0.0.1:3000".parse().unwrap())
        .serve(users_api.into_make_service())
        .await?;

    Ok(())
}
