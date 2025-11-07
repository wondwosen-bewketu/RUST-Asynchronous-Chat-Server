use std::env;

/// Database environment configuration
pub struct DatabaseConfig {
    pub url: String,
}

impl DatabaseConfig {
    /// Load database configuration from environment variables
    pub fn from_env() -> Self {
        let url = env::var("DATABASE_URL")
            .unwrap_or_else(|_| {
                // Fallback to component-based configuration
                let db_type = env::var("DATABASE_TYPE").unwrap_or_else(|_| "postgres".to_string());
                let host = env::var("DATABASE_HOST").expect("DATABASE_HOST must be set");
                let port = env::var("DATABASE_PORT").unwrap_or_else(|_| "5432".to_string());
                let username = env::var("DATABASE_USERNAME").expect("DATABASE_USERNAME must be set");
                let password = env::var("DATABASE_PASSWORD").expect("DATABASE_PASSWORD must be set");
                let name = env::var("DATABASE_NAME").expect("DATABASE_NAME must be set");
                
                format!("{}://{}:{}@{}:{}/{}", db_type, username, password, host, port, name)
            });
            
        Self { url }
    }
}