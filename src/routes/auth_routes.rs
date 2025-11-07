use axum::{
    routing::{get, post},
    Router,
    Json,
};
use axum::extract::State;
use axum::http::{StatusCode, HeaderMap};
use crate::modules::auth::dto::auth_dto::{RegisterDto, LoginDto, TokenResponse, RefreshTokenDto, ChangePasswordDto, UserResponse, ErrorResponse};
use crate::modules::auth::service::AuthService;
use sqlx::Pool;
use sqlx::Postgres;
use crate::config::environment::Environment;
use crate::modules::auth::utils::jwt::{JwtUtil, Claims};
use uuid::Uuid;
use utoipa::OpenApi;

/// RUST-AXUM-PROJECT API documentation
#[derive(OpenApi)]
#[openapi(
    paths(
        register,
        login,
        get_me,
        refresh_token,
        change_password,
        chat_websocket,
    ),
    components(
        schemas(RegisterDto, LoginDto, TokenResponse, RefreshTokenDto, ChangePasswordDto, UserResponse, ErrorResponse)
    ),
    tags(
        (name = "Authentication", description = "User authentication and management endpoints"),
        (name = "Chat", description = "Real-time chat endpoints")
    )
)]
pub struct ApiDoc;

/// Register a new user
#[utoipa::path(
    post,
    path = "/auth/register",
    request_body = RegisterDto,
    responses(
        (status = 201, description = "User registered successfully", body = serde_json::Value),
        (status = 400, description = "Invalid input", body = ErrorResponse),
        (status = 409, description = "User already exists", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse),
    ),
    tag = "Authentication"
)]
pub async fn register(
    State(pool): State<Pool<Postgres>>,
    Json(payload): Json<RegisterDto>,
) -> Result<Json<serde_json::Value>, (axum::http::StatusCode, String)> {
    let env = Environment::from_env();
    let auth_service = AuthService::new(pool, env);
    
    match auth_service.register(payload).await {
        Ok(user) => {
            let response = serde_json::json!({
                "success": true,
                "message": "User registered successfully",
                "user": user.to_response()
            });
            Ok(Json(response))
        }
        Err(e) => {
            if e.to_string().contains("already exists") {
                Err((axum::http::StatusCode::CONFLICT, e.to_string()))
            } else {
                Err((axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
            }
        }
    }
}

/// Login user
#[utoipa::path(
    post,
    path = "/auth/login",
    request_body = LoginDto,
    responses(
        (status = 200, description = "Login successful", body = TokenResponse),
        (status = 401, description = "Invalid credentials", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse),
    ),
    tag = "Authentication"
)]
pub async fn login(
    State(pool): State<Pool<Postgres>>,
    Json(payload): Json<LoginDto>,
) -> Result<Json<serde_json::Value>, (axum::http::StatusCode, String)> {
    let env = Environment::from_env();
    let auth_service = AuthService::new(pool, env);
    
    match auth_service.login(payload).await {
        Ok(token_response) => {
            let response = serde_json::json!({
                "success": true,
                "message": "Login successful",
                "token": token_response.token,
                "refresh_token": token_response.refresh_token,
                "expires_in": token_response.expires_in
            });
            Ok(Json(response))
        }
        Err(e) => {
            if e.to_string().contains("Invalid credentials") {
                Err((axum::http::StatusCode::UNAUTHORIZED, e.to_string()))
            } else {
                Err((axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
            }
        }
    }
}

/// Get current user profile
#[utoipa::path(
    post,
    path = "/auth/me",
    responses(
        (status = 200, description = "User profile retrieved successfully", body = UserResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse),
    ),
    tag = "Authentication"
)]
pub async fn get_me(
    State(pool): State<Pool<Postgres>>,
    headers: HeaderMap,
) -> Result<Json<serde_json::Value>, (axum::http::StatusCode, String)> {
    // Extract authorization header
    let auth_header = headers.get("authorization")
        .ok_or((StatusCode::UNAUTHORIZED, "Missing authorization header".to_string()))?
        .to_str()
        .map_err(|_| (StatusCode::UNAUTHORIZED, "Invalid authorization header".to_string()))?;

    if !auth_header.starts_with("Bearer ") {
        return Err((StatusCode::UNAUTHORIZED, "Invalid authorization header".to_string()));
    }

    let token = auth_header.trim_start_matches("Bearer ");
    let env = Environment::from_env();
    
    // Validate token
    let claims: Claims = JwtUtil::validate_access_token(token, &env.auth)
        .map_err(|_| (StatusCode::UNAUTHORIZED, "Invalid token".to_string()))?;

    // Extract user ID
    let user_id = Uuid::parse_str(&claims.sub)
        .map_err(|_| (StatusCode::UNAUTHORIZED, "Invalid user ID in token".to_string()))?;

    // Get user from database
    let auth_service = AuthService::new(pool, env);
    match auth_service.get_user_by_id(user_id).await {
        Ok(Some(user)) => {
            let response = serde_json::json!({
                "success": true,
                "user": user.to_response()
            });
            Ok(Json(response))
        }
        Ok(None) => Err((StatusCode::UNAUTHORIZED, "User not found".to_string())),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

/// Refresh access token
#[utoipa::path(
    post,
    path = "/auth/refresh-token",
    request_body = RefreshTokenDto,
    responses(
        (status = 200, description = "Token refreshed successfully", body = TokenResponse),
        (status = 401, description = "Invalid refresh token", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse),
    ),
    tag = "Authentication"
)]
pub async fn refresh_token(
    State(pool): State<Pool<Postgres>>,
    Json(payload): Json<RefreshTokenDto>,
) -> Result<Json<serde_json::Value>, (axum::http::StatusCode, String)> {
    let env = Environment::from_env();
    let auth_service = AuthService::new(pool, env);
    
    match auth_service.refresh_token(&payload.refresh_token).await {
        Ok(token_response) => {
            let response = serde_json::json!({
                "success": true,
                "token": token_response.token,
                "refresh_token": token_response.refresh_token,
                "expires_in": token_response.expires_in
            });
            Ok(Json(response))
        }
        Err(e) => {
            Err((StatusCode::UNAUTHORIZED, e.to_string()))
        }
    }
}

/// Change user password
#[utoipa::path(
    post,
    path = "/auth/change-password",
    request_body = ChangePasswordDto,
    responses(
        (status = 200, description = "Password changed successfully"),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 400, description = "Invalid input", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse),
    ),
    tag = "Authentication"
)]
pub async fn change_password(
    State(pool): State<Pool<Postgres>>,
    headers: HeaderMap,
    Json(payload): Json<ChangePasswordDto>,
) -> Result<Json<serde_json::Value>, (axum::http::StatusCode, String)> {
    // Extract authorization header
    let auth_header = headers.get("authorization")
        .ok_or((StatusCode::UNAUTHORIZED, "Missing authorization header".to_string()))?
        .to_str()
        .map_err(|_| (StatusCode::UNAUTHORIZED, "Invalid authorization header".to_string()))?;

    if !auth_header.starts_with("Bearer ") {
        return Err((StatusCode::UNAUTHORIZED, "Invalid authorization header".to_string()));
    }

    let token = auth_header.trim_start_matches("Bearer ");
    let env = Environment::from_env();
    
    // Validate token
    let claims: Claims = JwtUtil::validate_access_token(token, &env.auth)
        .map_err(|_| (StatusCode::UNAUTHORIZED, "Invalid token".to_string()))?;

    // Extract user ID
    let user_id = Uuid::parse_str(&claims.sub)
        .map_err(|_| (StatusCode::UNAUTHORIZED, "Invalid user ID in token".to_string()))?;

    // Change password
    let auth_service = AuthService::new(pool, env);
    match auth_service.change_password(user_id, payload).await {
        Ok(()) => {
            let response = serde_json::json!({
                "success": true,
                "message": "Password changed successfully"
            });
            Ok(Json(response))
        }
        Err(e) => {
            Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
        }
    }
}

/// WebSocket Chat Endpoint
/// 
/// Connect to the real-time chat server using WebSocket protocol.
/// 
/// Authentication is required using a valid JWT token.
/// 
/// You can provide the token in either:
/// 1. Query parameter: `?token=YOUR_JWT_TOKEN`
/// 2. Authorization header: `Authorization: Bearer YOUR_JWT_TOKEN`
/// 
/// You can also specify a room to join:
/// `?token=YOUR_JWT_TOKEN&room=room_name`
/// 
/// If no room is specified, you will join the default "general" room.
/// 
/// # WebSocket Communication
/// 
/// Once connected, you can send and receive messages:
/// 
/// ## Sending Messages
/// Simply send text messages through the WebSocket connection:
/// ```
/// Hello everyone!
/// ```
/// 
/// ## Receiving Messages
/// 
/// Messages are received in two formats:
/// 
/// 1. User Messages:
/// ```json
/// {
///   "user_id": "user-uuid",
///   "username": "User_xxxxxxxx",
///   "message": "Hello everyone!",
///   "timestamp": 1234567890
/// }
/// ```
/// 
/// 2. System Messages (prefixed with "system:"):
/// ```json
/// system:{"message":"User_xxxxxxxx has joined the chat.","timestamp":1234567890}
/// ```
/// 
/// # Authentication
/// 
/// All connections require a valid JWT token obtained through the authentication endpoints.
/// 
/// # Examples
/// 
/// JavaScript example:
/// ```javascript
/// const token = "YOUR_JWT_TOKEN";
/// const ws = new WebSocket(`ws://localhost:3005/ws?token=${token}`);
/// 
/// ws.onopen = () => {
///   console.log("Connected to chat server");
///   ws.send("Hello everyone!");
/// };
/// 
/// ws.onmessage = (event) => {
///   console.log("Received:", event.data);
/// };
/// ```
#[utoipa::path(
    get,
    path = "/ws",
    responses(
        (status = 101, description = "Switching to WebSocket protocol"),
        (status = 401, description = "Unauthorized - Invalid or missing JWT token"),
        (status = 404, description = "Not Found - WebSocket endpoint not found"),
    ),
    params(
        ("token" = String, Query, description = "JWT token for authentication (optional if provided in Authorization header)"),
        ("room" = String, Query, description = "Room name to join (optional, defaults to 'general')"),
    ),
    security(
        ("Authorization" = [])
    ),
    tag = "Chat"
)]
pub async fn chat_websocket() {}

/// Configure authentication routes
pub fn auth_routes() -> Router<Pool<Postgres>> {
    Router::new()
        .route("/auth/register", post(register))
        .route("/auth/login", post(login))
        .route("/auth/me", post(get_me))
        .route("/auth/refresh-token", post(refresh_token))
        .route("/auth/change-password", post(change_password))
        .route("/", get(|| async { "Rust Axum Project API Server is running!" }))
        .route("/health", get(|| async { "OK" }))
}