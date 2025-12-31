use axum::{
    routing::get,
    Router,
};

use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root_handler))
        .route("/health", get(health_check_hanlder));

    let addr = "0.0.0.0:3000";
    println!("Server Listening on {}", addr);

    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root_handler() -> &'static str {
    "Hello, Rust Server!"
}

async fn health_check_hanlder() -> &'static str {
    "OK"
}