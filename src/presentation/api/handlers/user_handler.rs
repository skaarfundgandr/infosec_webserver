use crate::application::repository::user_repo::UserRepo;
use axum::extract::Path;
use axum::response::IntoResponse;
use axum::{Json, http::StatusCode};

pub async fn get_by_id(Path(id): Path<i32>) -> impl IntoResponse {
    match UserRepo::get_by_id(id).await {
        Ok(Some(user)) => Json(user).into_response(),
        Ok(None) => StatusCode::NOT_FOUND.into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}
