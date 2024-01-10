use std::sync::Arc;

use axum::{debug_handler, extract::State, http::StatusCode, Json};

use crate::{
    domain::{dto::user_dto::CreateUserDTO, entities::User},
    services::user_service::Context,
    Event,
};

#[debug_handler]
pub async fn handler(
    State(user_service): State<Arc<Context>>,
    Json(payload): Json<CreateUserDTO>,
) -> Result<Json<User>, (StatusCode, String)> {
    let res = user_service.create_user(payload).await;
    if res.is_err() {
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            res.err().unwrap().to_string(),
        ));
    }

    let created_user = res.unwrap();
    let json_value = serde_json::to_string(&created_user).unwrap();
    user_service
        .tx
        .send(Event {
            event: "user.created".to_string(),
            payload: Some(json_value),
        })
        .await
        .unwrap();

    Ok(Json(created_user))
}
