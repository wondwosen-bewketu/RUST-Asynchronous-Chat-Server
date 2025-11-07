use axum::Router;
use dotenvy::dotenv;
use std::net::SocketAddr;
use tracing::info;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::config::environment::Environment;
use crate::infrastructure::db::init_pool;
use crate::routes::auth_routes;
use crate::utils::logger::init_logger;

mod config;
mod infrastructure;
mod modules;
mod routes;
mod utils;

// Import the ApiDoc from auth_routes
use crate::routes::auth_routes::ApiDoc;

#[tokio::main]
async fn main() {
    dotenv().ok();
    init_logger();

    let pool = init_pool().await;

    // Skip migrations for external database to avoid schema conflicts
    info!("Skipping migrations for external database - using existing schema");

    let app = Router::new()
        .merge(auth_routes())
        .merge(SwaggerUi::new("/swagger-ui/").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .with_state(pool);

    let env = Environment::from_env();
    let addr = SocketAddr::from(([0, 0, 0, 0], env.app.port));
    info!("Listening on {}", addr);
    info!("API Documentation available at: http://{}:{}/swagger-ui/", addr.ip(), addr.port());
    info!("Health check endpoint: http://{}:{}/health", addr.ip(), addr.port());

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}