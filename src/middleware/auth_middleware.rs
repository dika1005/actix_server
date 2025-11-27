use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpMessage, HttpResponse, body::EitherBody,
};
use futures_util::future::LocalBoxFuture;
use std::future::{ready, Ready};
use crate::utils::jwt;
use crate::dtos::common_dto::ApiResponse;

pub struct JwtMiddleware;

impl<S, B> Transform<S, ServiceRequest> for JwtMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type InitError = ();
    type Transform = JwtMiddlewareService<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(JwtMiddlewareService { service }))
    }
}

pub struct JwtMiddlewareService<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for JwtMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        // Extract token from Authorization header or cookie
        let token = req
            .headers()
            .get("Authorization")
            .and_then(|h| h.to_str().ok())
            .and_then(|h| {
                if h.starts_with("Bearer ") {
                    Some(h[7..].to_string())
                } else {
                    None
                }
            })
            .or_else(|| {
                // Fallback: cek cookie jika header tidak ada
                req.cookie("access_token")
                    .map(|c| c.value().to_string())
            });

        match token {
            Some(token) => {
                // Validate token
                match jwt::validate_token(&token) {
                    Ok(claims) => {
                        // Insert claims into request extensions for access in handlers
                        req.extensions_mut().insert(claims);
                        
                        let fut = self.service.call(req);
                        Box::pin(async move {
                            let res = fut.await?;
                            Ok(res.map_into_left_body())
                        })
                    }
                    Err(_) => {
                        let (request, _pl) = req.into_parts();
                        let response = HttpResponse::Unauthorized()
                            .json(ApiResponse::<()>::error("Invalid or expired token"))
                            .map_into_right_body();
                        Box::pin(async move {
                            Ok(ServiceResponse::new(request, response))
                        })
                    }
                }
            }
            None => {
                let (request, _pl) = req.into_parts();
                let response = HttpResponse::Unauthorized()
                    .json(ApiResponse::<()>::error("Missing authorization token"))
                    .map_into_right_body();
                Box::pin(async move {
                    Ok(ServiceResponse::new(request, response))
                })
            }
        }
    }
}

// Helper untuk extract claims dari request
pub mod extract {
    use actix_web::{HttpRequest, HttpResponse, HttpMessage};
    use crate::utils::jwt::Claims;
    use crate::dtos::common_dto::ApiResponse;

    #[allow(dead_code)]
    pub fn get_claims(req: &HttpRequest) -> Result<Claims, HttpResponse> {
        req.extensions()
            .get::<Claims>()
            .cloned()
            .ok_or_else(|| {
                HttpResponse::Unauthorized()
                    .json(ApiResponse::<()>::error("Unauthorized"))
            })
    }

    #[allow(dead_code)]
    pub fn require_admin(req: &HttpRequest) -> Result<Claims, HttpResponse> {
        let claims = get_claims(req)?;
        
        if claims.role != "admin" {
            return Err(HttpResponse::Forbidden()
                .json(ApiResponse::<()>::error("Admin access required")));
        }
        
        Ok(claims)
    }
}
