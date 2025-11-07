use std::env;

/// Authentication environment configuration
pub struct AuthConfig {
    pub jwt_secret: String,
    pub jwt_token_expires_in: String,
    pub refresh_secret: String,
    pub refresh_token_expires_in: String,
}

impl AuthConfig {
    /// Load authentication configuration from environment variables
    pub fn from_env() -> Self {
        let jwt_secret = env::var("AUTH_JWT_SECRET")
            .unwrap_or_else(|_| "32e8ce152d03053bc06535be4916345f04c874d86cadc39d5b9ef1570f5d39c2".to_string());
            
        let jwt_token_expires_in = env::var("AUTH_JWT_TOKEN_EXPIRES_IN")
            .unwrap_or_else(|_| "24hr".to_string());
            
        let refresh_secret = env::var("AUTH_REFRESH_SECRET")
            .unwrap_or_else(|_| "b82a645ec0ea881582aaabc931a7758c332c1c9e27fc1ae83ccd3006f76c90fb".to_string());
            
        let refresh_token_expires_in = env::var("AUTH_REFRESH_TOKEN_EXPIRES_IN")
            .unwrap_or_else(|_| "365d".to_string());
            
        Self {
            jwt_secret,
            jwt_token_expires_in,
            refresh_secret,
            refresh_token_expires_in,
        }
    }
}