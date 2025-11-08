use axum::{
    routing::post,
    Router,
    Json,
    extract::State,
    http::{StatusCode, HeaderMap},
};
use serde::{Deserialize, Serialize};
use sqlx::Pool;
use sqlx::Postgres;
use std::collections::HashMap;

use crate::modules::auth::utils::jwt::{JwtUtil, Claims};
use crate::modules::file_indexer::service::FileIndexerService;
use uuid::Uuid;
use utoipa::ToSchema;

/// Request payload for scanning files
#[derive(Deserialize, ToSchema)]
pub struct ScanRequest {
    /// Directory path to scan for duplicates (optional, defaults to /home/uploads)
    directory: Option<String>,
}

/// Response for file scan results
#[derive(Serialize, ToSchema)]
pub struct ScanResponse {
    /// Map of file hashes to lists of duplicate file paths
    duplicates: HashMap<String, Vec<String>>,
    /// Total number of files processed
    total_files: usize,
    /// Time taken to complete the scan
    time_taken: String,
}

/// Configure file routes
pub fn file_routes() -> Router<Pool<Postgres>> {
    Router::new()
        .route("/api/files/scan", post(scan_files))
}

/// Scan files for duplicates
/// 
/// Scans a directory for duplicate files using SHA256 hashing and multithreaded processing.
/// Only authenticated users can access this endpoint.
/// 
/// # Request Format
/// 
/// ```json
/// {
///   "directory": "/path/to/directory"
/// }
/// ```
/// 
/// # Response Format
/// 
/// ```json
/// {
///   "duplicates": {
///     "a1b2c3d4e5f6...": ["/path/file1.jpg", "/path/file2.jpg"],
///     "f6e5d4c3b2a1...": ["/path/doc1.pdf", "/path/doc2.pdf"]
///   },
///   "total_files": 150,
///   "time_taken": "1.8s"
/// }
/// ```
/// 
/// # Authentication
/// 
/// This endpoint requires a valid JWT token in the Authorization header:
/// `Authorization: Bearer YOUR_JWT_TOKEN`
#[utoipa::path(
    post,
    path = "/api/files/scan",
    request_body = ScanRequest,
    responses(
        (status = 200, description = "File scan completed successfully", body = ScanResponse),
        (status = 400, description = "Invalid request parameters", body = ErrorResponse),
        (status = 401, description = "Unauthorized - Invalid or missing JWT token", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse),
    ),
    security(
        ("Authorization" = [])
    ),
    tag = "File Management"
)]
pub async fn scan_files(
    State(_pool): State<Pool<Postgres>>,
    headers: HeaderMap,
    Json(payload): Json<ScanRequest>,
) -> Result<Json<ScanResponse>, (StatusCode, String)> {
    // Extract authorization header
    let auth_header = headers.get("authorization")
        .ok_or((StatusCode::UNAUTHORIZED, "Missing authorization header".to_string()))?
        .to_str()
        .map_err(|_| (StatusCode::UNAUTHORIZED, "Invalid authorization header".to_string()))?;

    if !auth_header.starts_with("Bearer ") {
        return Err((StatusCode::UNAUTHORIZED, "Invalid authorization header".to_string()));
    }

    let token = auth_header.trim_start_matches("Bearer ");
    let env = crate::config::environment::Environment::from_env();
    
    // Validate token
    let claims: Claims = JwtUtil::validate_access_token(token, &env.auth)
        .map_err(|_| (StatusCode::UNAUTHORIZED, "Invalid token".to_string()))?;

    // Extract user ID
    let _user_id = Uuid::parse_str(&claims.sub)
        .map_err(|_| (StatusCode::UNAUTHORIZED, "Invalid user ID in token".to_string()))?;

    // Determine directory to scan
    let directory = payload.directory.unwrap_or_else(|| "/home/uploads".to_string());
    
    // Validate directory path (basic security check)
    if directory.contains("..") {
        return Err((StatusCode::BAD_REQUEST, "Invalid directory path".to_string()));
    }
    
    // Perform the scan asynchronously
    let scan_result = FileIndexerService::scan_directory_async(&directory, Some(4))
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Scan failed: {}", e)))?;
    
    // Convert file paths to strings for JSON serialization
    let duplicates: HashMap<String, Vec<String>> = scan_result
        .duplicates
        .into_iter()
        .map(|(hash, paths)| {
            let path_strings: Vec<String> = paths
                .into_iter()
                .map(|path| path.to_string_lossy().to_string())
                .collect();
            (hash, path_strings)
        })
        .collect();
    
    let response = ScanResponse {
        duplicates,
        total_files: scan_result.total_files,
        time_taken: scan_result.time_taken,
    };
    
    Ok(Json(response))
}