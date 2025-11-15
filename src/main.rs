use axum::{middleware, routing::get, Router};
use gestion_inventario_backend::{
    db::connection::create_pool,
    routes::{auth, users},
    security::middleware::auth_middleware,
};
use std::net::SocketAddr;
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    // Load environment variables
    dotenvy::dotenv().ok();

    // Initialize tracing
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "example_tokio_postgres=debug,tower_http=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Create database pool
    let pool = create_pool().await.expect("Failed to create database pool");
    let pool = Arc::new(pool);

    // Run migrations
    sqlx::migrate!("./src/migrations")
        .run(&*pool)
        .await
        .expect("Failed to run database migrations");

    // Create CORS layer
    let cors = CorsLayer::new()
        .allow_methods(Any)
        .allow_origin(Any)
        .allow_headers(Any);

    // Create routers
    let auth_router = auth::create_router(Arc::clone(&pool));
    let users_public_router = users::create_public_router(Arc::clone(&pool));
    let users_protected_router = users::create_protected_router(Arc::clone(&pool))
        .route_layer(middleware::from_fn(auth_middleware));

    // Combine routers
    let app = Router::new()
        .route("/", get(|| async { "Backend is running 🚀" }))
        .merge(auth_router)
        .merge(users_public_router)
        .merge(users_protected_router)
        .layer(cors);

    // Run server
    let addr = SocketAddr::from(([0, 0, 0, 0], 8000));
    tracing::debug!("listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}