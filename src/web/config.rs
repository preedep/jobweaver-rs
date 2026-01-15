//! Web server configuration module
//!
//! This module defines configuration structures for the web server,
//! including authentication settings, JWT configuration, and Entra ID integration.

use serde::{Deserialize, Serialize};

/// Web server configuration
///
/// Contains all configuration settings for the web server including
/// network settings, database path, authentication, and security settings.
/// Values can be loaded from environment variables or use defaults.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebConfig {
    /// Server host address (e.g., "0.0.0.0" or "127.0.0.1")
    pub host: String,
    /// Server port number
    pub port: u16,
    /// Path to SQLite database file
    pub database_path: String,
    /// Secret key for JWT token signing and verification
    pub jwt_secret: String,
    /// Secret key for session management
    pub session_key: String,
    /// Whether Entra ID (Azure AD) authentication is enabled
    pub enable_entra_id: bool,
    /// Entra ID configuration (if enabled)
    pub entra_id_config: Option<EntraIdConfig>,
    
    // Local authentication settings
    /// Username for local authentication (loaded from .env)
    pub auth_username: String,
    /// Password for local authentication (loaded from .env)
    pub auth_password: String,
    /// Maximum number of failed login attempts before account lockout
    pub max_login_attempts: u32,
    /// Duration of account lockout in minutes after exceeding max attempts
    pub lockout_duration_minutes: u64,
}

/// Entra ID (Azure AD) OAuth configuration
///
/// Contains OAuth 2.0 credentials and settings for Entra ID authentication.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntraIdConfig {
    /// Azure AD application (client) ID
    pub client_id: String,
    /// Azure AD application client secret
    pub client_secret: String,
    /// Azure AD tenant ID
    pub tenant_id: String,
    /// OAuth redirect URI for callback
    pub redirect_uri: String,
}

impl Default for WebConfig {
    /// Creates a WebConfig with default values
    ///
    /// # Default Values
    ///
    /// - Host: "0.0.0.0" (all interfaces)
    /// - Port: 8080
    /// - Database: "controlm.db"
    /// - JWT Secret: "your-secret-key-change-in-production" (MUST change in production)
    /// - Session Key: "your-session-key-change-in-production" (MUST change in production)
    /// - Entra ID: Disabled
    /// - Auth Username: "admin"
    /// - Auth Password: "admin" (MUST change in production)
    /// - Max Login Attempts: 3
    /// - Lockout Duration: 30 minutes
    ///
    /// # Security Warning
    ///
    /// Default credentials and secrets MUST be changed in production environments.
    /// Load values from .env file for proper security.
    fn default() -> Self {
        Self {
            host: "0.0.0.0".to_string(),
            port: 8080,
            database_path: "controlm.db".to_string(),
            jwt_secret: "your-secret-key-change-in-production".to_string(),
            session_key: "your-session-key-change-in-production".to_string(),
            enable_entra_id: false,
            entra_id_config: None,
            auth_username: "admin".to_string(),
            auth_password: "admin".to_string(),
            max_login_attempts: 3,
            lockout_duration_minutes: 30,
        }
    }
}

impl WebConfig {
    /// Creates a new WebConfig with specified database path
    ///
    /// Other settings use default values.
    ///
    /// # Arguments
    ///
    /// * `database_path` - Path to SQLite database file
    ///
    /// # Returns
    ///
    /// WebConfig instance with defaults except for database path
    pub fn new(database_path: String) -> Self {
        Self {
            database_path,
            ..Default::default()
        }
    }

    /// Sets the server port
    ///
    /// # Arguments
    ///
    /// * `port` - Port number to bind to
    ///
    /// # Returns
    ///
    /// Self for method chaining
    pub fn with_port(mut self, port: u16) -> Self {
        self.port = port;
        self
    }

    /// Sets the server host address
    ///
    /// # Arguments
    ///
    /// * `host` - Host address to bind to (e.g., "0.0.0.0" or "127.0.0.1")
    ///
    /// # Returns
    ///
    /// Self for method chaining
    pub fn with_host(mut self, host: String) -> Self {
        self.host = host;
        self
    }

    /// Enables Entra ID authentication with the given configuration
    ///
    /// # Arguments
    ///
    /// * `config` - Entra ID OAuth configuration
    ///
    /// # Returns
    ///
    /// Self for method chaining
    pub fn with_entra_id(mut self, config: EntraIdConfig) -> Self {
        self.enable_entra_id = true;
        self.entra_id_config = Some(config);
        self
    }
}
