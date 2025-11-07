use crate::config::environment::Environment;
use sqlx::{PgPool, Pool, Postgres};

/// Initialize the database connection pool
pub async fn init_pool() -> Pool<Postgres> {
    let env = Environment::from_env();
    
    PgPool::connect(&env.database.url)
        .await
        .expect("Failed to create database pool")
}