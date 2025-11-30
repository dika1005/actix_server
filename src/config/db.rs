use sea_orm::{DatabaseConnection, Database, ConnectOptions};
use std::env;
use dotenvy::dotenv;
use std::time::Duration;

pub async fn init_db() -> DatabaseConnection {
    dotenv().ok();
    let url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    
    println!("Connecting to database...");
    
    let mut opt = ConnectOptions::new(url);
    opt.max_connections(5)
        .min_connections(1)
        .connect_timeout(Duration::from_secs(30))
        .acquire_timeout(Duration::from_secs(30))
        .idle_timeout(Duration::from_secs(600))
        .max_lifetime(Duration::from_secs(3600))
        .sqlx_logging(true)
        .sqlx_logging_level(log::LevelFilter::Debug);
    
    match Database::connect(opt).await {
        Ok(db) => {
            println!("Database connected successfully!");
            db
        }
        Err(e) => {
            eprintln!("Failed to connect to database: {:?}", e);
            panic!("Database connection failed: {:?}", e);
        }
    }
}