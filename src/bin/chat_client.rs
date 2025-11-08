use futures::{SinkExt, StreamExt};
use std::env;
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message as TungsteniteMessage};
use url::Url;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get command line arguments
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 3 {
        eprintln!("Usage: {} <server_url> <jwt_token> [room]", args[0]);
        eprintln!("Example: {} http://localhost:3005 eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9... [general]", args[0]);
        std::process::exit(1);
    }
    
    let server_url = &args[1];
    let jwt_token = &args[2];
    let room = args.get(3).map(|s| s.as_str()).unwrap_or("general");
    
    // Construct the WebSocket URL with query parameters
    let ws_url = format!(
        "{}/ws?token={}&room={}",
        server_url.replace("http://", "ws://").replace("https://", "wss://"),
        jwt_token,
        room
    );

    println!("Connecting to chat server at {}...", ws_url);
    
    // Connect to the WebSocket server
    let url = Url::parse(&ws_url)?;
    let (ws_stream, _) = connect_async(url).await?;
    let (mut sender, mut receiver) = ws_stream.split();
    
    println!("Connected to room '{}'! You can start sending messages. Type 'quit' to exit.", room);
    
    // Spawn a task to listen for incoming messages
    let recv_handle = tokio::spawn(async move {
        while let Some(Ok(msg)) = receiver.next().await {
            match msg {
                TungsteniteMessage::Text(text) => {
                    println!("Received: {}", text);
                }
                TungsteniteMessage::Close(_) => {
                    println!("Connection closed by server");
                    break;
                }
                _ => {}
            }
        }
    });
    
    // Read user input and send messages
    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        let input = input.trim();
        
        if input == "quit" {
            break;
        }
        
        if !input.is_empty() {
            if let Err(e) = sender.send(TungsteniteMessage::Text(input.to_string())).await {
                eprintln!("Failed to send message: {}", e);
                break;
            }
        }
    }
    
    // Close the connection
    sender.close().await?;
    
    // Wait for the receiver task to finish
    recv_handle.await?;
    
    println!("Disconnected from chat server");
    Ok(())
}