use std::error::Error;

use axum::Router;
use tracing::info;

use crate::{api::users::initialize_users_api, repository::DbManager, services::user_service};

mod api;
mod domain;
mod infrastructure;
mod repository;
mod services;

pub async fn run() -> Result<(), Box<dyn Error>> {
    let db_conn = infrastructure::database::connect().await?;
    let manager = DbManager::new(db_conn.clone());

    let user_ctx = user_service::Context::new(manager);
    let users_api = initialize_users_api(user_ctx);

    let app = Router::new().nest("/users", users_api);

    info!("API starting @ port http://127.0.0.1:3000");
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();

    Ok(())
}
