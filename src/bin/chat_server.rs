use axum::{
    routing::get,
    Router,
};
use rust_axum_project::modules::chat::server::{websocket_handler, ChatState};
use std::net::SocketAddr;
use tracing_subscriber;

#[tokio::main]
async fn main() {
    // Initialize logging
    tracing_subscriber::fmt::init();

    // Initialize chat state
    let chat_state = ChatState::new();

    // Build our application with the chat route
    let app = Router::new()
        .route("/ws", get(websocket_handler))
        .with_state(chat_state);

    // Run it
    let addr = SocketAddr::from(([127, 0, 0, 1], 3001));
    println!("Chat server running on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}