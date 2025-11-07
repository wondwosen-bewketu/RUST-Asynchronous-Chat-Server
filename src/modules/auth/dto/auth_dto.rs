use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct RegisterDto {
    #[schema(example = "John Doe")]
    pub full_name: String,
    #[schema(example = "john@example.com")]
    pub email: String,
    #[schema(example = 30, minimum = 1, maximum = 120)]
    pub age: i32,
    #[schema(example = "password123")]
    pub password: String,
    #[serde(with = "time::serde::rfc3339")]
    #[schema(example = "1993-05-15T00:00:00Z", value_type = String)]
    pub date_of_birth: OffsetDateTime,
    #[schema(example = "male")]
    pub gender: String,
    #[schema(example = "+1234567890")]
    pub phone_number: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct LoginDto {
    #[schema(example = "john@example.com")]
    pub email: String,
    #[schema(example = "password123")]
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct TokenResponse {
    #[schema(example = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...")]
    pub token: String,
    #[schema(example = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...")]
    pub refresh_token: String,
    #[schema(example = 3600)]
    pub expires_in: i64,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct RefreshTokenDto {
    #[schema(example = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...")]
    pub refresh_token: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ChangePasswordDto {
    #[schema(example = "oldpassword123")]
    pub old_password: String,
    #[schema(example = "newpassword456")]
    pub new_password: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UserResponse {
    pub id: String,
    pub name: String,
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ErrorResponse {
    #[schema(example = "Invalid credentials")]
    pub error: String,
}