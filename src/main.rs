mod config;
mod entity;
mod dtos;
mod handlers;
mod routes;

use actix_web::{web, App, HttpServer};
use dotenvy::dotenv;
use std::env;
use handlers::user_handler::AppState; // Pastikan struct AppState di-public di handler

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 1. Load Environment Variables (.env)
    dotenv().ok();
    
    // 2. Setup Database
    // Pastikan function establish_connection sudah ada di src/config/db.rs
    let db = config::db::init_db().await;
    
    // 3. Simpan DB ke dalam State
    // Ini biar database bisa diakses dari semua handler/routes
    let state = web::Data::new(AppState { db });

    // 4. Ambil Host & Port dari .env
    let host = env::var("SERVER_HOST").unwrap_or("127.0.0.1".to_string());
    let port = env::var("SERVER_PORT").unwrap_or("8080".to_string());
    let address = format!("{}:{}", host, port);

    println!("Server running at http://{}", address);

    // 5. Jalanin Server
    HttpServer::new(move || {
        App::new()
            // Inject state ke aplikasi (PENTING!)
            .app_data(state.clone()) 
            // Load routes
            .configure(routes::config) 
    })
    .bind(&address)? // Bind ke alamat dinamis dari variable
    .run()
    .await
}