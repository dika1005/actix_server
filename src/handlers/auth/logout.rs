use actix_web::{HttpResponse, Responder, cookie::{Cookie, time::Duration as CookieDuration}};
use crate::dtos::common_dto::ApiResponse;

pub async fn logout() -> impl Responder {
    // Clear cookies dengan set max_age ke 0
    let access_cookie = Cookie::build("access_token", "")
        .path("/")
        .http_only(true)
        .max_age(CookieDuration::ZERO)
        .finish();

    let refresh_cookie = Cookie::build("refresh_token", "")
        .path("/")
        .http_only(true)
        .max_age(CookieDuration::ZERO)
        .finish();

    HttpResponse::Ok()
        .cookie(access_cookie)
        .cookie(refresh_cookie)
        .json(ApiResponse::<()>::success("Logout successful", ()))
}
