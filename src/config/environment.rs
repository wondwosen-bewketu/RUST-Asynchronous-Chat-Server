use crate::config::env::{AppConfig, AuthConfig, DatabaseConfig};

/// Application environment configuration
pub struct Environment {
    pub database: DatabaseConfig,
    pub app: AppConfig,
    pub auth: AuthConfig,
}

impl Environment {
    /// Load environment configuration from environment variables
    pub fn from_env() -> Self {
        let database = DatabaseConfig::from_env();
        let app = AppConfig::from_env();
        let auth = AuthConfig::from_env();
        
        Self {
            database,
            app,
            auth,
        }
    }
}