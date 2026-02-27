use crate::presentation::api::handlers::user_handler;
use axum::Router;
use axum::routing::get;

pub fn routes() -> Router {
    Router::new().route("/user={id}", get(user_handler::get_by_id))
}
