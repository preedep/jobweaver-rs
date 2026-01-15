//! Authentication module
//!
//! This module provides JWT-based authentication services including token generation,
//! validation, password hashing, and user management.

use actix_web::{dev::ServiceRequest, Error, HttpMessage};
use actix_web::error::ErrorUnauthorized;
use actix_web_httpauth::extractors::bearer::BearerAuth;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use chrono::{Duration, Utc, DateTime};
use bcrypt::{hash, verify, DEFAULT_COST};
use std::collections::HashMap;
use std::sync::Mutex;

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

/// Login attempt tracking information
#[derive(Debug, Clone)]
struct LoginAttempt {
    /// Number of failed attempts
    attempts: u32,
    /// Timestamp when the account was locked (if locked)
    locked_until: Option<DateTime<Utc>>,
}

/// Login attempt tracker for rate limiting
///
/// Tracks failed login attempts per username and enforces lockout periods.
pub struct LoginAttemptTracker {
    /// Map of username to login attempt info
    attempts: Mutex<HashMap<String, LoginAttempt>>,
    /// Maximum allowed failed attempts before lockout
    max_attempts: u32,
    /// Lockout duration in minutes
    lockout_duration_minutes: u64,
}

impl LoginAttemptTracker {
    /// Creates a new LoginAttemptTracker
    ///
    /// # Arguments
    ///
    /// * `max_attempts` - Maximum failed attempts before lockout
    /// * `lockout_duration_minutes` - Duration of lockout in minutes
    pub fn new(max_attempts: u32, lockout_duration_minutes: u64) -> Self {
        Self {
            attempts: Mutex::new(HashMap::new()),
            max_attempts,
            lockout_duration_minutes,
        }
    }

    /// Checks if a username is currently locked out
    ///
    /// # Arguments
    ///
    /// * `username` - Username to check
    ///
    /// # Returns
    ///
    /// `true` if locked out, `false` otherwise
    pub fn is_locked_out(&self, username: &str) -> bool {
        let mut attempts = self.attempts.lock().unwrap();
        
        if let Some(attempt) = attempts.get(username) {
            if let Some(locked_until) = attempt.locked_until {
                if Utc::now() < locked_until {
                    return true;
                } else {
                    // Lockout period expired, reset attempts
                    attempts.remove(username);
                    return false;
                }
            }
        }
        false
    }

    /// Records a failed login attempt
    ///
    /// # Arguments
    ///
    /// * `username` - Username that failed to login
    ///
    /// # Returns
    ///
    /// Number of remaining attempts before lockout (0 if locked out)
    pub fn record_failed_attempt(&self, username: &str) -> u32 {
        let mut attempts = self.attempts.lock().unwrap();
        
        let attempt = attempts.entry(username.to_string()).or_insert(LoginAttempt {
            attempts: 0,
            locked_until: None,
        });
        
        attempt.attempts += 1;
        
        if attempt.attempts >= self.max_attempts {
            // Lock the account
            let lockout_until = Utc::now() + Duration::minutes(self.lockout_duration_minutes as i64);
            attempt.locked_until = Some(lockout_until);
            return 0;
        }
        
        self.max_attempts - attempt.attempts
    }

    /// Resets login attempts for a username (called on successful login)
    ///
    /// # Arguments
    ///
    /// * `username` - Username to reset attempts for
    pub fn reset_attempts(&self, username: &str) {
        let mut attempts = self.attempts.lock().unwrap();
        attempts.remove(username);
    }

    /// Gets the remaining time until lockout expires
    ///
    /// # Arguments
    ///
    /// * `username` - Username to check
    ///
    /// # Returns
    ///
    /// Remaining minutes until unlock, or None if not locked
    pub fn get_lockout_remaining_minutes(&self, username: &str) -> Option<i64> {
        let attempts = self.attempts.lock().unwrap();
        
        if let Some(attempt) = attempts.get(username) {
            if let Some(locked_until) = attempt.locked_until {
                let now = Utc::now();
                if now < locked_until {
                    let remaining = locked_until.signed_duration_since(now);
                    return Some(remaining.num_minutes() + 1); // Round up
                }
            }
        }
        None
    }
}

/// In-memory user store for local authentication
///
/// Validates credentials against configuration from .env file.
#[derive(Debug)]
pub struct UserStore {
    /// Configured username from .env
    username: String,
    /// Configured password hash from .env
    password_hash: String,
}

impl UserStore {
    /// Creates a new UserStore with credentials from configuration
    ///
    /// # Arguments
    ///
    /// * `username` - Configured username from .env
    /// * `password` - Configured password from .env (will be hashed)
    ///
    /// # Returns
    ///
    /// A new UserStore instance
    pub fn new(username: String, password: String) -> Self {
        let auth_service = AuthService::new("temp-secret".to_string());
        let password_hash = auth_service.hash_password(&password)
            .expect("Failed to hash password");
        
        Self { 
            username,
            password_hash,
        }
    }

    /// Verifies user credentials against configured username and password
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
        if username != self.username {
            return false;
        }
        
        let auth_service = AuthService::new("temp-secret".to_string());
        auth_service.verify_password(password, &self.password_hash).unwrap_or(false)
    }
}

