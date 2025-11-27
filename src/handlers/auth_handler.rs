use actix_web::{web, HttpResponse, Responder};
use sea_orm::*;
use crate::entity::user;
use crate::dtos::auth_dto::{RegisterRequest, LoginRequest, AuthResponse, UserInfo};
use crate::dtos::common_dto::ApiResponse;
use crate::utils::{hash, jwt};
use crate::handlers::user_handler::AppState;

// Handler untuk register user baru
pub async fn register(
    data: web::Data<AppState>,
    req_body: web::Json<RegisterRequest>,
) -> impl Responder {
    // Validasi input
    if req_body.username.trim().is_empty() {
        return HttpResponse::BadRequest().json(ApiResponse::<()>::error("Username cannot be empty"));
    }

    if req_body.email.trim().is_empty() || !req_body.email.contains('@') {
        return HttpResponse::BadRequest().json(ApiResponse::<()>::error("Invalid email format"));
    }

    if req_body.password.len() < 6 {
        return HttpResponse::BadRequest().json(ApiResponse::<()>::error("Password must be at least 6 characters"));
    }

    // Cek apakah email sudah terdaftar
    match user::Entity::find()
        .filter(user::Column::Email.eq(&req_body.email))
        .one(&data.db)
        .await
    {
        Ok(Some(_)) => {
            return HttpResponse::Conflict().json(ApiResponse::<()>::error("Email already registered"));
        }
        Err(err) => {
            eprintln!("Database error: {:?}", err);
            return HttpResponse::InternalServerError().json(ApiResponse::<()>::error("Database error"));
        }
        _ => {}
    }

    // Hash password
    let password_hash = match hash::hash_password(&req_body.password) {
        Ok(hash) => hash,
        Err(err) => {
            eprintln!("Hash error: {:?}", err);
            return HttpResponse::InternalServerError().json(ApiResponse::<()>::error("Failed to process password"));
        }
    };

    // Buat user baru
    let new_user = user::ActiveModel {
        id: NotSet,
        username: Set(req_body.username.trim().to_string()),
        email: Set(req_body.email.trim().to_lowercase()),
        password_hash: Set(password_hash),
        role: Set("user".to_string()), // Default role
        created_at: NotSet,
        updated_at: NotSet,
    };

    // Insert ke database
    match user::Entity::insert(new_user).exec(&data.db).await {
        Ok(result) => {
            let user_id = result.last_insert_id;

            // Query ulang untuk mendapatkan data lengkap
            match user::Entity::find_by_id(user_id).one(&data.db).await {
                Ok(Some(user_model)) => {
                    // Generate JWT token
                    match jwt::generate_token(user_model.id, user_model.email.clone(), user_model.role.clone()) {
                        Ok(token) => {
                            let response = AuthResponse {
                                token,
                                user: UserInfo {
                                    id: user_model.id,
                                    username: user_model.username,
                                    email: user_model.email,
                                    role: user_model.role,
                                },
                            };
                            HttpResponse::Created().json(ApiResponse::success("User registered successfully", response))
                        }
                        Err(err) => {
                            eprintln!("JWT error: {:?}", err);
                            HttpResponse::InternalServerError().json(ApiResponse::<()>::error("Failed to generate token"))
                        }
                    }
                }
                Ok(None) => HttpResponse::InternalServerError().json(ApiResponse::<()>::error("User created but not found")),
                Err(err) => {
                    eprintln!("Database error: {:?}", err);
                    HttpResponse::InternalServerError().json(ApiResponse::<()>::error("Database error"))
                }
            }
        }
        Err(err) => {
            eprintln!("Error creating user: {:?}", err);
            HttpResponse::InternalServerError().json(ApiResponse::<()>::error("Failed to create user"))
        }
    }
}

// Handler untuk login
pub async fn login(
    data: web::Data<AppState>,
    req_body: web::Json<LoginRequest>,
) -> impl Responder {
    // Validasi input
    if req_body.email.trim().is_empty() {
        return HttpResponse::BadRequest().json(ApiResponse::<()>::error("Email cannot be empty"));
    }

    if req_body.password.is_empty() {
        return HttpResponse::BadRequest().json(ApiResponse::<()>::error("Password cannot be empty"));
    }

    // Cari user berdasarkan email
    match user::Entity::find()
        .filter(user::Column::Email.eq(req_body.email.trim().to_lowercase()))
        .one(&data.db)
        .await
    {
        Ok(Some(user_model)) => {
            // Verify password
            match hash::verify_password(&req_body.password, &user_model.password_hash) {
                Ok(true) => {
                    // Password benar, generate token
                    match jwt::generate_token(user_model.id, user_model.email.clone(), user_model.role.clone()) {
                        Ok(token) => {
                            let response = AuthResponse {
                                token,
                                user: UserInfo {
                                    id: user_model.id,
                                    username: user_model.username,
                                    email: user_model.email,
                                    role: user_model.role,
                                },
                            };
                            HttpResponse::Ok().json(ApiResponse::success("Login successful", response))
                        }
                        Err(err) => {
                            eprintln!("JWT error: {:?}", err);
                            HttpResponse::InternalServerError().json(ApiResponse::<()>::error("Failed to generate token"))
                        }
                    }
                }
                Ok(false) => {
                    HttpResponse::Unauthorized().json(ApiResponse::<()>::error("Invalid email or password"))
                }
                Err(err) => {
                    eprintln!("Password verification error: {:?}", err);
                    HttpResponse::InternalServerError().json(ApiResponse::<()>::error("Authentication error"))
                }
            }
        }
        Ok(None) => {
            HttpResponse::Unauthorized().json(ApiResponse::<()>::error("Invalid email or password"))
        }
        Err(err) => {
            eprintln!("Database error: {:?}", err);
            HttpResponse::InternalServerError().json(ApiResponse::<()>::error("Database error"))
        }
    }
}
