use std::error::Error;

use axum::Router;
use tokio::{signal, spawn, sync::mpsc};
use tracing::info;

use crate::{api::users::initialize_users_api, repository::DbManager, services::user_service};

mod api;
mod domain;
mod infrastructure;
mod repository;
mod services;

pub struct Event {
    pub event: String,
    pub payload: Option<String>,
}

pub async fn run() -> Result<(), Box<dyn Error>> {
    let db_conn = infrastructure::database::connect().await?;
    let manager = DbManager::new(db_conn.clone());

    let (tx, mut rx) = mpsc::channel::<Event>(50);

    let user_ctx = user_service::Context::new(manager, tx);
    let users_api = initialize_users_api(user_ctx);

    let app = Router::new().nest("/users", users_api);

    info!("API starting @ port http://127.0.0.1:3000");
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    let worker = spawn(async move {
        while let Some(event) = rx.recv().await {
            println!("{:?}", event.event);
            println!("{:?}", event.payload.unwrap())
        }
    });

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    worker.abort();
    Ok(())
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {
            println!("running!")
        },
        _ = terminate => {},
    }
}
