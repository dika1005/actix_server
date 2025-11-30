use actix_web::web;
use crate::handlers::auth;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .route("/register", web::post().to(auth::register))
            .route("/login", web::post().to(auth::login))
            .route("/logout", web::post().to(auth::logout))
    );
}
