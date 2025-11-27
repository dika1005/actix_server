use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};
use serde::{Deserialize, Serialize};
use chrono::{Utc, Duration};
use std::env;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,      // user_id
    pub email: String,
    pub role: String,
    pub exp: i64,         // expiry timestamp
    pub iat: i64,         // issued at
}

impl Claims {
    pub fn new(user_id: i32, email: String, role: String) -> Self {
        let now = Utc::now();
        let exp = (now + Duration::hours(24)).timestamp();
        
        Self {
            sub: user_id.to_string(),
            email,
            role,
            exp,
            iat: now.timestamp(),
        }
    }
}

pub fn generate_token(user_id: i32, email: String, role: String) -> Result<String, jsonwebtoken::errors::Error> {
    let claims = Claims::new(user_id, email, role);
    let secret = env::var("JWT_SECRET").unwrap_or_else(|_| "your-secret-key-change-in-production".to_string());
    
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
}

pub fn validate_token(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let secret = env::var("JWT_SECRET").unwrap_or_else(|_| "your-secret-key-change-in-production".to_string());
    
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )?;
    
    Ok(token_data.claims)
}
