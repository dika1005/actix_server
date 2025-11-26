// src/routes/user.rs
use actix_web::web;
use crate::handlers::user_handler;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users")
            // POST /api/users -> create_user
            .route("", web::post().to(user_handler::create_user)) 
    );
}