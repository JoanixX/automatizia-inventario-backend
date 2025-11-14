use axum::{routing::get, Router};
use std::net::SocketAddr;
use tracing_subscriber;

#[tokio::main]
async fn main() {
    // Inicializar logs
    tracing_subscriber::fmt().init();

    // Enrutador básico
    let app = Router::new().route("/", get(|| async { "Backend funcionando 🚀" }));

    // Dirección
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Servidor corriendo en http://{}", addr);

    // Método moderno para correr Axum 0.7
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}