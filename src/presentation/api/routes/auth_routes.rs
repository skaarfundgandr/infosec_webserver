use crate::presentation::api::handlers::auth_handler;
use axum::Router;
use axum::routing::post;

pub fn routes() -> Router {
    Router::new().route("/login", post(auth_handler::login))
}
