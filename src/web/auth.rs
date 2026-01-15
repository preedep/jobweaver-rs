//! Authentication module
//!
//! This module provides JWT-based authentication services including token generation,
//! validation, password hashing, and user management.

use actix_web::{dev::ServiceRequest, Error, HttpMessage};
use actix_web::error::ErrorUnauthorized;
use actix_web_httpauth::extractors::bearer::BearerAuth;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use chrono::{Duration, Utc};
use bcrypt::{hash, verify, DEFAULT_COST};

use crate::web::models::{AuthType, UserInfo};

/// JWT claims structure
///
/// Contains user information and token expiration embedded in JWT tokens.
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    /// Subject (user identifier)
    pub sub: String,
    /// Username
    pub username: String,
    /// Display name for UI
    pub display_name: String,
    /// Authentication type (Local or EntraId)
    pub auth_type: AuthType,
    /// Token expiration timestamp (Unix epoch)
    pub exp: i64,
}

/// Service for handling authentication operations
///
/// Provides JWT token generation/validation and password hashing/verification.
pub struct AuthService {
    /// Secret key for JWT signing and verification
    jwt_secret: String,
}

impl AuthService {
    /// Creates a new AuthService instance
    ///
    /// # Arguments
    ///
    /// * `jwt_secret` - Secret key for JWT operations
    ///
    /// # Returns
    ///
    /// A new AuthService instance
    pub fn new(jwt_secret: String) -> Self {
        Self { jwt_secret }
    }

    /// Generates a JWT token for a user
    ///
    /// Token is valid for 24 hours from creation.
    ///
    /// # Arguments
    ///
    /// * `user` - User information to embed in the token
    ///
    /// # Returns
    ///
    /// Result containing the JWT token string or an error
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

    /// Verifies and decodes a JWT token
    ///
    /// # Arguments
    ///
    /// * `token` - JWT token string to verify
    ///
    /// # Returns
    ///
    /// Result containing the decoded Claims or an error
    pub fn verify_token(&self, token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
        decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.jwt_secret.as_bytes()),
            &Validation::default(),
        )
        .map(|data| data.claims)
    }

    /// Hashes a password using bcrypt
    ///
    /// # Arguments
    ///
    /// * `password` - Plain text password to hash
    ///
    /// # Returns
    ///
    /// Result containing the hashed password or an error
    pub fn hash_password(&self, password: &str) -> Result<String, bcrypt::BcryptError> {
        hash(password, DEFAULT_COST)
    }

    /// Verifies a password against a bcrypt hash
    ///
    /// # Arguments
    ///
    /// * `password` - Plain text password to verify
    /// * `hash` - Bcrypt hash to verify against
    ///
    /// # Returns
    ///
    /// Result containing true if password matches, false otherwise
    pub fn verify_password(&self, password: &str, hash: &str) -> Result<bool, bcrypt::BcryptError> {
        verify(password, hash)
    }
}

/// Middleware validator for JWT bearer tokens
///
/// Validates the JWT token and injects claims into request extensions.
///
/// # Arguments
///
/// * `req` - Service request to validate
/// * `credentials` - Bearer token credentials
///
/// # Returns
///
/// Result containing the validated request or an error
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

/// In-memory user store for local authentication
///
/// Stores username to password hash mappings. In production, this should
/// be replaced with a proper database backend.
#[derive(Debug)]
pub struct UserStore {
    /// Map of username to bcrypt password hash
    users: std::collections::HashMap<String, String>,
}

impl UserStore {
    /// Creates a new UserStore with a default admin user
    ///
    /// Default credentials: username="admin", password="admin"
    ///
    /// # Returns
    ///
    /// A new UserStore instance
    pub fn new() -> Self {
        let mut users = std::collections::HashMap::new();
        
        let auth_service = AuthService::new("temp-secret".to_string());
        if let Ok(hashed) = auth_service.hash_password("admin") {
            users.insert("admin".to_string(), hashed);
        }
        
        Self { users }
    }

    /// Verifies user credentials
    ///
    /// # Arguments
    ///
    /// * `username` - Username to verify
    /// * `password` - Plain text password to verify
    ///
    /// # Returns
    ///
    /// `true` if credentials are valid, `false` otherwise
    pub fn verify_user(&self, username: &str, password: &str) -> bool {
        if let Some(hash) = self.users.get(username) {
            let auth_service = AuthService::new("temp-secret".to_string());
            auth_service.verify_password(password, hash).unwrap_or(false)
        } else {
            false
        }
    }

    /// Adds a new user to the store
    ///
    /// # Arguments
    ///
    /// * `username` - Username for the new user
    /// * `password_hash` - Bcrypt hash of the user's password
    pub fn add_user(&mut self, username: String, password_hash: String) {
        self.users.insert(username, password_hash);
    }
}

impl Default for UserStore {
    fn default() -> Self {
        Self::new()
    }
}
