use crate::models::user::{NewUser, UpdateUser};
use crate::services::users::{
    create_user, delete_user, get_user, list_users, update_user,
};
use crate::utils::error::ApiError;
use crate::AppState;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;

pub fn create_public_router(pool: Arc<PgPool>) -> Router {
    let app_state = AppState { pool };
    Router::new()
        .route("/users", post(create_user_handler))
        .with_state(app_state)
}

pub fn create_protected_router(pool: Arc<PgPool>) -> Router {
    let app_state = AppState { pool };
    Router::new()
        .route("/users", get(list_users_handler))
        .route(
            "/users/:id",
            get(get_user_handler)
                .put(update_user_handler)
                .delete(delete_user_handler),
        )
        .with_state(app_state)
}

async fn create_user_handler(
    State(state): State<AppState>,
    Json(new_user): Json<NewUser>,
) -> Result<impl IntoResponse, ApiError> {
    match create_user(&state.pool, new_user).await {
        Ok(user) => Ok((StatusCode::CREATED, Json(user))),
        Err(e) => Err(e.into()),
    }
}

async fn list_users_handler(
    State(state): State<AppState>,
) -> Result<impl IntoResponse, ApiError> {
    match list_users(&state.pool).await {
        Ok(users) => Ok(Json(users)),
        Err(e) => Err(e.into()),
    }
}

async fn get_user_handler(
    State(state): State<AppState>,
    Path(user_id): Path<Uuid>,
) -> Result<impl IntoResponse, ApiError> {
    match get_user(&state.pool, user_id).await {
        Ok(Some(user)) => Ok(Json(user)),
        Ok(None) => Err(ApiError::NotFound),
        Err(e) => Err(e.into()),
    }
}

async fn update_user_handler(
    State(state): State<AppState>,
    Path(user_id): Path<Uuid>,
    Json(payload): Json<UpdateUser>, // <-- Renombrada a "payload"
) -> Result<impl IntoResponse, ApiError> {
    match update_user(&state.pool, user_id, payload).await { // <-- Arreglado
        Ok(user) => Ok(Json(user)),
        Err(e) => {
            if e.to_string().contains("User not found") {
                Err(ApiError::NotFound)
            } else {
                Err(e.into())
            }
        }
    }
}

async fn delete_user_handler(
    State(state): State<AppState>,
    Path(user_id): Path<Uuid>,
) -> Result<impl IntoResponse, ApiError> {
    match delete_user(&state.pool, user_id).await {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(e) => {
            if e.to_string().contains("User not found") {
                Err(ApiError::NotFound)
            } else {
                Err(e.into())
            }
        }
    }
}
