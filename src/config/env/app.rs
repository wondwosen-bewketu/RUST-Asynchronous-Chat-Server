use std::env;

/// Application environment configuration
pub struct AppConfig {
    pub port: u16,
}

impl AppConfig {
    /// Load application configuration from environment variables
    pub fn from_env() -> Self {
        let port = env::var("PORT")
            .unwrap_or_else(|_| "3005".to_string())
            .parse::<u16>()
            .unwrap_or(3005);
            
        Self { port }
    }
}