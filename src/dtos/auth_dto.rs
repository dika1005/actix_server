use serde::{Deserialize, Serialize};

// Request untuk register
#[derive(Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub email: String,
    pub password: String,
}

// Request untuk login
#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

// Response setelah berhasil register (tanpa token)
#[derive(Serialize)]
pub struct RegisterResponse {
    pub user: UserInfo,
}

// Response setelah berhasil login (dengan access & refresh token)
#[derive(Serialize)]
pub struct LoginResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub token_type: String,
    pub expires_in: i64,
    pub user: UserInfo,
}

#[derive(Serialize)]
pub struct UserInfo {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub role: String,
}
