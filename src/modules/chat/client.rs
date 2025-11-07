//! Simple chat client for testing the WebSocket server

use futures::{SinkExt, StreamExt};
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message as TungsteniteMessage};
use url::Url;

/// Connect to the chat server
pub async fn connect_to_chat_server(
    server_url: &str,
    jwt_token: &str,
    room: Option<&str>,
) -> Result<
    (
        futures::stream::SplitSink<
            tokio_tungstenite::WebSocketStream<
                tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>,
            >,
            TungsteniteMessage,
        >,
        futures::stream::SplitStream<
            tokio_tungstenite::WebSocketStream<
                tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>,
            >,
        >,
    ),
    Box<dyn std::error::Error>,
> {
    let room_param = room.unwrap_or("general");
    let ws_url = format!(
        "{}/ws?token={}&room={}",
        server_url.replace("http://", "ws://").replace("https://", "wss://"),
        jwt_token,
        room_param
    );

    let url = Url::parse(&ws_url)?;
    let (ws_stream, _) = connect_async(url).await?;
    Ok(ws_stream.split())
}

/// Send a message through the WebSocket connection
pub async fn send_message(
    sender: &mut futures::stream::SplitSink<
        tokio_tungstenite::WebSocketStream<
            tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>,
        >,
        TungsteniteMessage,
    >,
    message: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    sender.send(TungsteniteMessage::Text(message.to_string())).await?;
    Ok(())
}