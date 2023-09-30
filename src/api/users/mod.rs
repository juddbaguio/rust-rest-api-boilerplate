use std::sync::Arc;

use axum::{routing::*, Router};

use crate::services::user_service::Context;

mod create_user;

pub fn initialize_users_api(ctx: Arc<Context>) -> Router {
    Router::new()
        .route("/", post(create_user::handler))
        .with_state(ctx)
}
