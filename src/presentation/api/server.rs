use crate::presentation::api::routes::{auth_routes, user_routes};
use axum::routing::get;
use axum::{Router, serve};
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tokio::signal;
use tokio::signal::unix;
use tokio::signal::unix::SignalKind;
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
        .with_graceful_shutdown(shutdown_signal())
        .await
        .expect("Failed to start server!");
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        unix::signal(SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}
