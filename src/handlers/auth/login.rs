use actix_web::{web, HttpResponse, Responder, cookie::{Cookie, time::Duration as CookieDuration}};
use sea_orm::*;
use crate::entity::user;
use crate::dtos::auth_dto::{LoginRequest, LoginResponse, UserInfo};
use crate::dtos::common_dto::ApiResponse;
use crate::utils::{hash, jwt};
use crate::handlers::user_handler::AppState;

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
                    // Password benar, generate access & refresh token
                    let access_token = jwt::generate_access_token(
                        user_model.id,
                        user_model.email.clone(),
                        user_model.role.clone(),
                    );
                    let refresh_token = jwt::generate_refresh_token(
                        user_model.id,
                        user_model.email.clone(),
                        user_model.role.clone(),
                    );

                    match (access_token, refresh_token) {
                        (Ok(access), Ok(refresh)) => {
                            // Set cookies untuk access & refresh token
                            let access_cookie = Cookie::build("access_token", access.clone())
                                .path("/")
                                .http_only(true)
                                .secure(false) // Set true di production dengan HTTPS
                                .max_age(CookieDuration::hours(1))
                                .finish();

                            let refresh_cookie = Cookie::build("refresh_token", refresh.clone())
                                .path("/")
                                .http_only(true)
                                .secure(false) // Set true di production dengan HTTPS
                                .max_age(CookieDuration::days(7))
                                .finish();

                            let response = LoginResponse {
                                access_token: access,
                                refresh_token: refresh,
                                token_type: "Bearer".to_string(),
                                expires_in: 3600, // 1 jam dalam detik
                                user: UserInfo {
                                    id: user_model.id,
                                    username: user_model.username,
                                    email: user_model.email,
                                    role: user_model.role,
                                },
                            };

                            HttpResponse::Ok()
                                .cookie(access_cookie)
                                .cookie(refresh_cookie)
                                .json(ApiResponse::success("Login successful", response))
                        }
                        _ => {
                            eprintln!("JWT generation error");
                            HttpResponse::InternalServerError().json(ApiResponse::<()>::error("Failed to generate tokens"))
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
