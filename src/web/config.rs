use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebConfig {
    pub host: String,
    pub port: u16,
    pub database_path: String,
    pub jwt_secret: String,
    pub session_key: String,
    pub enable_entra_id: bool,
    pub entra_id_config: Option<EntraIdConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntraIdConfig {
    pub client_id: String,
    pub client_secret: String,
    pub tenant_id: String,
    pub redirect_uri: String,
}

impl Default for WebConfig {
    fn default() -> Self {
        Self {
            host: "127.0.0.1".to_string(),
            port: 8080,
            database_path: "controlm.db".to_string(),
            jwt_secret: "your-secret-key-change-in-production".to_string(),
            session_key: "your-session-key-change-in-production".to_string(),
            enable_entra_id: false,
            entra_id_config: None,
        }
    }
}

impl WebConfig {
    pub fn new(database_path: String) -> Self {
        Self {
            database_path,
            ..Default::default()
        }
    }

    pub fn with_port(mut self, port: u16) -> Self {
        self.port = port;
        self
    }

    pub fn with_host(mut self, host: String) -> Self {
        self.host = host;
        self
    }

    pub fn with_entra_id(mut self, config: EntraIdConfig) -> Self {
        self.enable_entra_id = true;
        self.entra_id_config = Some(config);
        self
    }
}
