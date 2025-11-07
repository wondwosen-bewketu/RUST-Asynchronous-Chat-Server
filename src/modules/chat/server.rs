use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        Query, State,
    },
    http::{StatusCode, header},
    response::Response,
};
use futures::{sink::SinkExt, stream::StreamExt};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};
use tokio::sync::broadcast;
use uuid::Uuid;

use crate::modules::auth::utils::jwt::JwtUtil;

type RoomName = String;
type UserName = String; 
type UserId = Uuid;

#[derive(Debug, Clone)]
pub struct ConnectedUser {
    pub username: UserName,
}

#[derive(Debug, Clone)]
pub struct ChatState {
    pub connected_users: Arc<Mutex<HashMap<UserId, ConnectedUser>>>,
    pub rooms: Arc<Mutex<HashMap<RoomName, broadcast::Sender<Message>>>>,
}

impl ChatState {
    pub fn new() -> Self {
        Self {
            connected_users: Arc::new(Mutex::new(HashMap::new())),
            rooms: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn add_user_to_room(&self, user_id: UserId, username: UserName, _room_name: RoomName) -> Result<(), String> {
        let mut users = self.connected_users.lock().unwrap();
        users.insert(
            user_id,
            ConnectedUser {
                username,
            },
        );
        Ok(())
    }

    pub fn remove_user(&self, user_id: UserId) -> Option<ConnectedUser> {
        let mut users = self.connected_users.lock().unwrap();
        users.remove(&user_id)
    }

    pub fn get_room_broadcaster(&self, room_name: &str) -> broadcast::Sender<Message> {
        let mut rooms = self.rooms.lock().unwrap();
        if let Some(sender) = rooms.get(room_name) {
            sender.clone()
        } else {
            let (sender, _receiver) = broadcast::channel(100);
            rooms.insert(room_name.to_string(), sender.clone());
            sender
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct ConnectionQuery {
    pub token: Option<String>,
    pub room: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatMessage {
    pub user_id: String,
    pub username: String,
    pub message: String,
    pub timestamp: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SystemMessage {
    pub message: String,
    pub timestamp: u64,
}

pub async fn websocket_handler(
    ws: WebSocketUpgrade,
    Query(query): Query<ConnectionQuery>,
    State(state): State<ChatState>,
    headers: header::HeaderMap,
) -> Result<Response, StatusCode> {
    // Extract token from query parameter or Authorization header
    let token = if let Some(token) = query.token {
        println!("Using token from query parameter");
        token
    } else if let Some(auth_header) = headers.get("Authorization") {
        if let Ok(auth_str) = auth_header.to_str() {
            if auth_str.starts_with("Bearer ") {
                println!("Using token from Authorization header");
                auth_str[7..].to_string()
            } else {
                eprintln!("Invalid Authorization header format");
                return Err(StatusCode::UNAUTHORIZED);
            }
        } else {
            eprintln!("Failed to parse Authorization header");
            return Err(StatusCode::UNAUTHORIZED);
        }
    } else {
        eprintln!("No token provided in query or Authorization header");
        return Err(StatusCode::UNAUTHORIZED);
    };

    // Validate JWT token
    let env = crate::config::environment::Environment::from_env();
    println!("Attempting to validate token: {}", &token[..std::cmp::min(token.len(), 20)]); // Log first 20 chars of token
    let claims = JwtUtil::validate_access_token(&token, &env.auth);
    
    match claims {
        Ok(claims) => {
            println!("Token validation successful");
            let user_id = Uuid::parse_str(&claims.sub)
                .map_err(|_| {
                    eprintln!("Failed to parse user ID from token");
                    StatusCode::UNAUTHORIZED
                })?;

            let username = format!("User_{}", &user_id.to_string()[..8]);
            let room_name = query.room.unwrap_or_else(|| "general".to_string());

            println!("User {} connecting to room {}", username, room_name);

            Ok(ws.on_upgrade(move |socket| handle_socket(socket, state, user_id, username, room_name)))
        }
        Err(e) => {
            eprintln!("Token validation failed: {:?}", e);
            Err(StatusCode::UNAUTHORIZED)
        }
    }
}

async fn handle_socket(
    socket: WebSocket,
    state: ChatState,
    user_id: UserId,
    username: UserName,
    room_name: RoomName,
) {
    if let Err(e) = state.add_user_to_room(user_id, username.clone(), room_name.clone()) {
        eprintln!("Failed to add user to room: {}", e);
        return;
    }

    let room_sender = state.get_room_broadcaster(&room_name);
    let mut room_receiver = room_sender.subscribe();

    let join_msg = SystemMessage {
        message: format!("{} has joined the chat.", username),
        timestamp: std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs(),
    };
    
    let join_msg_json = serde_json::to_string(&join_msg).unwrap();
    let _ = room_sender.send(Message::Text(format!("system:{}", join_msg_json)));

    let (mut sender, mut receiver) = socket.split();

    let room_sender_clone = room_sender.clone();
    let username_clone = username.clone();

    let mut send_task = tokio::spawn(async move {
        while let Ok(msg) = room_receiver.recv().await {
            if sender.send(msg).await.is_err() {
                break;
            }
        }
    });

    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(msg)) = receiver.next().await {
            match msg {
                Message::Text(text) => {
                    let chat_msg = ChatMessage {
                        user_id: user_id.to_string(),
                        username: username_clone.clone(),
                        message: text,
                        timestamp: std::time::SystemTime::now()
                            .duration_since(std::time::UNIX_EPOCH)
                            .unwrap()
                            .as_secs(),
                    };
                    
                    let chat_msg_json = serde_json::to_string(&chat_msg).unwrap();
                    let _ = room_sender_clone.send(Message::Text(chat_msg_json));
                }
                Message::Close(_) => {
                    break;
                }
                _ => {}
            }
        }
    });

    tokio::select! {
        _ = (&mut send_task) => {
            recv_task.abort();
        },
        _ = (&mut recv_task) => {
            send_task.abort();
        }
    }

    if let Some(user) = state.remove_user(user_id) {
        let leave_msg = SystemMessage {
            message: format!("{} has left the chat.", user.username),
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        };
        
        let leave_msg_json = serde_json::to_string(&leave_msg).unwrap();
        let _ = room_sender.send(Message::Text(format!("system:{}", leave_msg_json)));
    }

    println!("User {} disconnected from room {}", username, room_name);
}