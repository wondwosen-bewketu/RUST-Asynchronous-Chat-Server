use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey, errors::Result as JwtResult};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // Subject (user ID)
    pub exp: usize,  // Expiration time (as UTC timestamp)
    pub iat: usize,  // Issued at time (as UTC timestamp)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RefreshClaims {
    pub sub: String, // Subject (user ID)
    pub exp: usize,  // Expiration time (as UTC timestamp)
    pub iat: usize,  // Issued at time (as UTC timestamp)
}

pub struct JwtUtil;

impl JwtUtil {
    pub fn generate_access_token(user_id: String, auth_config: &crate::config::env::AuthConfig) -> Result<String, Box<dyn std::error::Error>> {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)?
            .as_secs();
            
        // Parse expiration time (default to 24 hours)
        let expires_in_seconds = match auth_config.jwt_token_expires_in.as_str() {
            s if s.ends_with("hr") => {
                let hours: u64 = s.trim_end_matches("hr").parse().unwrap_or(24);
                hours * 3600
            },
            s if s.ends_with("d") => {
                let days: u64 = s.trim_end_matches("d").parse().unwrap_or(1);
                days * 24 * 3600
            },
            s => s.parse().unwrap_or(24 * 3600),
        };
        
        let claims = Claims {
            sub: user_id,
            exp: (now + expires_in_seconds) as usize,
            iat: now as usize,
        };

        let token = encode(&Header::default(), &claims, &EncodingKey::from_secret(auth_config.jwt_secret.as_ref()))?;
        Ok(token)
    }

    pub fn generate_refresh_token(user_id: String, auth_config: &crate::config::env::AuthConfig) -> Result<String, Box<dyn std::error::Error>> {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)?
            .as_secs();
            
        // Parse expiration time (default to 365 days)
        let expires_in_seconds = match auth_config.refresh_token_expires_in.as_str() {
            s if s.ends_with("d") => {
                let days: u64 = s.trim_end_matches("d").parse().unwrap_or(365);
                days * 24 * 3600
            },
            s if s.ends_with("hr") => {
                let hours: u64 = s.trim_end_matches("hr").parse().unwrap_or(365 * 24);
                hours * 3600
            },
            s => s.parse().unwrap_or(365 * 24 * 3600),
        };
        
        let claims = RefreshClaims {
            sub: user_id,
            exp: (now + expires_in_seconds) as usize,
            iat: now as usize,
        };

        let token = encode(&Header::default(), &claims, &EncodingKey::from_secret(auth_config.refresh_secret.as_ref()))?;
        Ok(token)
    }

    pub fn validate_access_token(token: &str, auth_config: &crate::config::env::AuthConfig) -> JwtResult<Claims> {
        let validation = Validation::default();
        decode::<Claims>(token, &DecodingKey::from_secret(auth_config.jwt_secret.as_ref()), &validation)
            .map(|data| data.claims)
    }

    pub fn validate_refresh_token(token: &str, auth_config: &crate::config::env::AuthConfig) -> JwtResult<RefreshClaims> {
        let validation = Validation::default();
        decode::<RefreshClaims>(token, &DecodingKey::from_secret(auth_config.refresh_secret.as_ref()), &validation)
            .map(|data| data.claims)
    }
}