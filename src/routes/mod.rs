use actix_web::web;

pub mod user;
pub mod auth;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .configure(user::config)
            .configure(auth::config)
    );
}
