use sea_orm::DatabaseConnection;
use sea_orm::Database;
use std::env;
use dotenvy::dotenv;

pub async fn init_db() -> DatabaseConnection {
    dotenv().ok();
    let url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    Database::connect(&url).await.expect("Failed to connect to the database")
}