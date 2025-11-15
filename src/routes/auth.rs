use crate::services::auth::{login, LoginRequest};
use crate::utils::error::ApiError;
use crate::AppState;
use axum::{extract::State, http::StatusCode, response::IntoResponse, routing::post, Json, Router};
use sqlx::PgPool;
use std::sync::Arc;

pub fn create_router(pool: Arc<PgPool>) -> Router {
    let app_state = AppState { pool };
    Router::new()
        .route("/login", post(login_handler))
        .with_state(app_state)
}

async fn login_handler(
    State(state): State<AppState>,
    Json(req): Json<LoginRequest>,
) -> Result<impl IntoResponse, ApiError> {
    match login(&state.pool, req).await {
        Ok(token) => Ok((StatusCode::OK, Json(serde_json::json!({ "token": token })))),
        Err(_) => Err(ApiError::Unauthorized),
    }
}
