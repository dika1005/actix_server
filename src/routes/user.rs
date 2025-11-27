use actix_web::web;
use crate::handlers::user_handler;
use crate::middleware::auth_middleware::JwtMiddleware;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users")
            // Protected endpoint - requires JWT
            .wrap(JwtMiddleware)
            .route("", web::post().to(user_handler::create_user)) 
    );
}