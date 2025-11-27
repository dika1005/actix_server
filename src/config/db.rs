use sea_orm::{DatabaseConnection, Database, ConnectOptions};
use std::env;
use dotenvy::dotenv;
use std::time::Duration;

pub async fn init_db() -> DatabaseConnection {
    dotenv().ok();
    let url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    
    let mut opt = ConnectOptions::new(url);
    opt.max_connections(10)
        .min_connections(1)
        .connect_timeout(Duration::from_secs(10))
        .acquire_timeout(Duration::from_secs(10))
        .idle_timeout(Duration::from_secs(300))
        .max_lifetime(Duration::from_secs(3600))
        .sqlx_logging(true);
    
    Database::connect(opt)
        .await
        .expect("Failed to connect to the database")
}