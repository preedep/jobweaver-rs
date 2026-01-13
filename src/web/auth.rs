use actix_web::{dev::ServiceRequest, Error, HttpMessage};
use actix_web::error::ErrorUnauthorized;
use actix_web_httpauth::extractors::bearer::BearerAuth;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use chrono::{Duration, Utc};
use bcrypt::{hash, verify, DEFAULT_COST};

use crate::web::models::{AuthType, UserInfo};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub username: String,
    pub display_name: String,
    pub auth_type: AuthType,
    pub exp: i64,
}

pub struct AuthService {
    jwt_secret: String,
}

impl AuthService {
    pub fn new(jwt_secret: String) -> Self {
        Self { jwt_secret }
    }

    pub fn generate_token(&self, user: &UserInfo) -> Result<String, jsonwebtoken::errors::Error> {
        let expiration = Utc::now()
            .checked_add_signed(Duration::hours(24))
            .expect("valid timestamp")
            .timestamp();

        let claims = Claims {
            sub: user.username.clone(),
            username: user.username.clone(),
            display_name: user.display_name.clone(),
            auth_type: user.auth_type.clone(),
            exp: expiration,
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.jwt_secret.as_bytes()),
        )
    }

    pub fn verify_token(&self, token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
        decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.jwt_secret.as_bytes()),
            &Validation::default(),
        )
        .map(|data| data.claims)
    }

    pub fn hash_password(&self, password: &str) -> Result<String, bcrypt::BcryptError> {
        hash(password, DEFAULT_COST)
    }

    pub fn verify_password(&self, password: &str, hash: &str) -> Result<bool, bcrypt::BcryptError> {
        verify(password, hash)
    }
}

pub async fn validator(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    let config = req.app_data::<actix_web::web::Data<crate::web::WebConfig>>()
        .expect("WebConfig not found");
    
    let auth_service = AuthService::new(config.jwt_secret.clone());
    
    match auth_service.verify_token(credentials.token()) {
        Ok(claims) => {
            req.extensions_mut().insert(claims);
            Ok(req)
        }
        Err(_) => Err((ErrorUnauthorized("Invalid token"), req)),
    }
}

#[derive(Debug)]
pub struct UserStore {
    users: std::collections::HashMap<String, String>,
}

impl UserStore {
    pub fn new() -> Self {
        let mut users = std::collections::HashMap::new();
        
        let auth_service = AuthService::new("temp-secret".to_string());
        if let Ok(hashed) = auth_service.hash_password("admin") {
            users.insert("admin".to_string(), hashed);
        }
        
        Self { users }
    }

    pub fn verify_user(&self, username: &str, password: &str) -> bool {
        if let Some(hash) = self.users.get(username) {
            let auth_service = AuthService::new("temp-secret".to_string());
            auth_service.verify_password(password, hash).unwrap_or(false)
        } else {
            false
        }
    }

    pub fn add_user(&mut self, username: String, password_hash: String) {
        self.users.insert(username, password_hash);
    }
}

impl Default for UserStore {
    fn default() -> Self {
        Self::new()
    }
}
