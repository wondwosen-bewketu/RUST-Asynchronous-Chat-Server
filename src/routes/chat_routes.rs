use axum::{
    routing::get,
    Router,
};
use sqlx::Pool;
use sqlx::Postgres;

use crate::modules::chat::server::{websocket_handler, ChatState};

/// Configure chat routes
pub fn chat_routes(chat_state: ChatState) -> Router<Pool<Postgres>> {
    Router::new()
        .route("/ws", get(websocket_handler))
        .with_state(chat_state)
}
