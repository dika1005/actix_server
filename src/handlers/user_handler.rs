use actix_web::{web, HttpResponse, Responder};
use sea_orm::*;
use crate::entity::user;
use crate::dtos::user_dto::{CreateUserRequest, UserResponse, ErrorResponse};

// AppState untuk menyimpan database connection
pub struct AppState {
    pub db: DatabaseConnection,
}

// Handler Create User
pub async fn create_user(
    data: web::Data<AppState>,
    req_body: web::Json<CreateUserRequest>,
) -> impl Responder {
    // Validasi input
    if req_body.name.trim().is_empty() {
        return HttpResponse::BadRequest().json(ErrorResponse {
            error: "Name cannot be empty".to_string(),
        });
    }

    if req_body.email.trim().is_empty() || !req_body.email.contains('@') {
        return HttpResponse::BadRequest().json(ErrorResponse {
            error: "Invalid email format".to_string(),
        });
    }

    // Siapkan data untuk insert
    let new_user = user::ActiveModel {
        id: NotSet,
        username: Set(req_body.name.trim().to_string()),
        email: Set(req_body.email.trim().to_lowercase()),
    };

    // Insert ke database
    match user::Entity::insert(new_user).exec(&data.db).await {
        Ok(result) => {
            let user_id = result.last_insert_id;

            // Query ulang untuk mendapatkan data lengkap
            match user::Entity::find_by_id(user_id).one(&data.db).await {
                Ok(Some(user_model)) => HttpResponse::Created().json(UserResponse {
                    id: user_model.id,
                    username: user_model.username,
                    email: user_model.email,
                }),
                Ok(None) => HttpResponse::InternalServerError().json(ErrorResponse {
                    error: "User created but not found".to_string(),
                }),
                Err(err) => {
                    eprintln!("Error fetching created user: {:?}", err);
                    HttpResponse::InternalServerError().json(ErrorResponse {
                        error: "Database error".to_string(),
                    })
                }
            }
        }
        Err(err) => {
            eprintln!("Error creating user: {:?}", err);
            HttpResponse::InternalServerError().json(ErrorResponse {
                error: "Failed to create user".to_string(),
            })
        }
    }
}