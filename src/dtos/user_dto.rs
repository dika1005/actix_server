use serde::{Deserialize, Serialize};

// Request DTO untuk create user
#[derive(Deserialize)]
pub struct CreateUserRequest {
    pub name: String,
    pub email: String,
}

// Response DTO untuk user
#[derive(Serialize)]
pub struct UserResponse {
    pub id: i32,
    pub username: String,
    pub email: String,
}

// Response DTO untuk error
#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: String,
}
