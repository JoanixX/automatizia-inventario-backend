use sqlx::PgPool;
use std::sync::Arc;

pub mod config;
pub mod db;
pub mod models;
pub mod routes;
pub mod security;
pub mod services;
pub mod utils;

#[derive(Clone)]
pub struct AppState {
    pub pool: Arc<PgPool>,
}
