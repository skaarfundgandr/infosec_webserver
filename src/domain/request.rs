use serde::Deserialize;

#[derive(Deserialize)]
pub struct LoginRequest {
    pub query: String,
}

#[derive(Deserialize)]
pub struct UserQuery {
    pub user_id: i32,
}
