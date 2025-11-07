use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use time::OffsetDateTime;

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    // Only including the fields we know exist in the database
    #[serde(with = "time::serde::rfc3339")]
    pub created_at: OffsetDateTime,
    #[serde(with = "time::serde::rfc3339")]
    pub updated_at: OffsetDateTime,
}

impl User {
    pub fn to_response(&self) -> crate::modules::auth::dto::auth_dto::UserResponse {
        crate::modules::auth::dto::auth_dto::UserResponse {
            id: self.id.to_string(),
            name: self.name.clone(),
            email: self.email.clone(),
        }
    }
}