use crate::{security::token::decode_token, utils::error::ApiError};
use axum::{
    extract::State,
    http::{header, StatusCode},
    middleware::Next,
    response::Response,
    http::Request,
};
use std::sync::Arc;
use axum::body::Body;
use sqlx::PgPool;

pub async fn auth_middleware(
    mut req: Request<Body>, // <-- Especifica el tipo del cuerpo
    next: Next,
) -> Result<Response, StatusCode> {
    let token = req.headers()
        .get(header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok())
        .and_then(|header| header.strip_prefix("Bearer "));

    match token {
        Some(token) => {
            match decode_token(token) {
                Ok(claims) => {
                    req.extensions_mut().insert(claims);
                    Ok(next.run(req).await)
                }
                Err(_) => Err(StatusCode::UNAUTHORIZED),
            }
        }
        None => Err(StatusCode::UNAUTHORIZED),
    }
}
