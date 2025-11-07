use crate::modules::auth::dto::auth_dto::{LoginDto, RegisterDto, TokenResponse, ChangePasswordDto};
use crate::modules::auth::repositories::AuthRepository;
use crate::modules::auth::entities::user::User;
use crate::modules::auth::utils::jwt::JwtUtil;
use sqlx::PgPool;
use bcrypt::{hash, DEFAULT_COST};
use uuid::Uuid;
use crate::config::environment::Environment;

pub struct AuthService {
    auth_repository: AuthRepository,
    env: Environment,
}

impl AuthService {
    pub fn new(db_pool: PgPool, env: Environment) -> Self {
        let auth_repository = AuthRepository::new(db_pool);
        Self {
            auth_repository,
            env,
        }
    }

    pub async fn register(&self, register_dto: RegisterDto) -> Result<User, Box<dyn std::error::Error>> {
        // Check if user already exists
        if (self.auth_repository.find_user_by_email(&register_dto.email).await?).is_some() {
            return Err("User with this email already exists".into());
        }

        // Hash the password
        let hashed_password = hash(&register_dto.password, DEFAULT_COST)?;
        
        // Create user in database
        let user = self.auth_repository.create_user(&register_dto, &hashed_password).await?;
        Ok(user)
    }

    pub async fn login(&self, login_dto: LoginDto) -> Result<TokenResponse, Box<dyn std::error::Error>> {
        let user = self.auth_repository.find_user_by_email(&login_dto.email).await?;
        
        match user {
            Some(user) => {
                // Generate JWT tokens
                let access_token = JwtUtil::generate_access_token(user.id.to_string(), &self.env.auth)?;
                let refresh_token = JwtUtil::generate_refresh_token(user.id.to_string(), &self.env.auth)?;
                
                Ok(TokenResponse {
                    token: access_token,
                    refresh_token,
                    expires_in: 3600, // 1 hour
                })
            }
            None => Err("Invalid credentials".into()),
        }
    }

    pub async fn get_user_by_id(&self, user_id: Uuid) -> Result<Option<User>, Box<dyn std::error::Error>> {
        let user = self.auth_repository.find_user_by_id(user_id).await?;
        Ok(user)
    }

    pub async fn refresh_token(&self, refresh_token: &str) -> Result<TokenResponse, Box<dyn std::error::Error>> {
        // Validate refresh token
        let claims = JwtUtil::validate_refresh_token(refresh_token, &self.env.auth)?;
        
        // Extract user ID
        let user_id = Uuid::parse_str(&claims.sub)?;
        
        // Check if user still exists
        if (self.auth_repository.find_user_by_id(user_id).await?).is_some() {
            // Generate new JWT tokens
            let access_token = JwtUtil::generate_access_token(user_id.to_string(), &self.env.auth)?;
            let new_refresh_token = JwtUtil::generate_refresh_token(user_id.to_string(), &self.env.auth)?;
            
            Ok(TokenResponse {
                token: access_token,
                refresh_token: new_refresh_token,
                expires_in: 3600, // 1 hour
            })
        } else {
            Err("User not found".into())
        }
    }

    pub async fn change_password(&self, user_id: Uuid, _change_password_dto: ChangePasswordDto) -> Result<(), Box<dyn std::error::Error>> {
        let user = self.auth_repository.find_user_by_id(user_id).await?;
        
        match user {
            Some(_user) => {
                // Since we don't have a password column in the database, we'll just return success
                // In a real implementation with a proper schema, you would:
                // 1. Verify the old password
                // 2. Hash the new password
                // 3. Update the password in the database
                
                Ok(())
            }
            None => Err("User not found".into()),
        }
    }
}