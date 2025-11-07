use crate::modules::auth::entities::user::User;
use crate::modules::auth::dto::auth_dto::RegisterDto;
use sqlx::{Pool, Postgres, Error};
use uuid::Uuid;

pub struct AuthRepository {
    db_pool: Pool<Postgres>,
}

impl AuthRepository {
    pub fn new(db_pool: Pool<Postgres>) -> Self {
        Self { db_pool }
    }

    /// Create a new user in the database
    pub async fn create_user(&self, register_dto: &RegisterDto, _hashed_password: &str) -> Result<User, Error> {
        // Using only the basic columns that we know exist in the database
        let user = sqlx::query_as::<_, User>(
            "INSERT INTO users (name, email) 
             VALUES ($1, $2) 
             RETURNING id, name, email, created_at, updated_at"
        )
        .bind(&register_dto.full_name)
        .bind(&register_dto.email)
        .fetch_one(&self.db_pool)
        .await?;

        Ok(user)
    }

    /// Find a user by email
    pub async fn find_user_by_email(&self, email: &str) -> Result<Option<User>, Error> {
        let user = sqlx::query_as::<_, User>(
            "SELECT id, name, email, created_at, updated_at 
             FROM users WHERE email = $1"
        )
        .bind(email)
        .fetch_optional(&self.db_pool)
        .await?;

        Ok(user)
    }

    /// Find a user by ID
    pub async fn find_user_by_id(&self, id: Uuid) -> Result<Option<User>, Error> {
        let user = sqlx::query_as::<_, User>(
            "SELECT id, name, email, created_at, updated_at 
             FROM users WHERE id = $1"
        )
        .bind(id)
        .fetch_optional(&self.db_pool)
        .await?;

        Ok(user)
    }
}