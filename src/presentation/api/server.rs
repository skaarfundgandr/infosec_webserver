use crate::presentation::api::routes::{auth_routes, user_routes};
use axum::routing::get;
use axum::{Router, serve};
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tower_http::cors::{Any, CorsLayer};

pub async fn start() {
    let cors_layer = CorsLayer::new().allow_origin(Any).allow_headers(Any);

    let router = Router::new()
        .route("/health", get(|| async { "OK" }))
        .nest("/auth", auth_routes::routes())
        .nest("/user", user_routes::routes())
        .with_state::<()>(())
        .layer(cors_layer);

    let listener = TcpListener::bind(SocketAddr::from(([0, 0, 0, 0], 8080)))
        .await
        .expect("Failed to bind to address!");

    println!("Attempting to start server on port 8080...");

    serve(listener, router)
        .await
        .expect("Failed to start server!");
}
