use futures::{SinkExt, StreamExt};
use rust_axum_project::modules::chat::client::{connect_to_chat_server, send_message};
use std::env;
use tokio_tungstenite::tungstenite::protocol::Message as TungsteniteMessage;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get command line arguments
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 3 {
        eprintln!("Usage: {} <server_url> <jwt_token> [room]", args[0]);
        std::process::exit(1);
    }
    
    let server_url = &args[1];
    let jwt_token = &args[2];
    let room = args.get(3).map(|s| s.as_str());
    
    println!("Connecting to chat server at {}...", server_url);
    
    // Connect to the chat server
    let (mut sender, mut receiver) = connect_to_chat_server(server_url, jwt_token, room).await?;
    
    println!("Connected! You can start sending messages. Type 'quit' to exit.");
    
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
            if let Err(e) = send_message(&mut sender, input).await {
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