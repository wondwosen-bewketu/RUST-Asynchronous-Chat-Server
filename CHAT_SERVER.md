# Asynchronous Chat Server Documentation

## Overview

This document provides comprehensive documentation for the asynchronous chat server implementation. The chat server is built using Rust with Tokio for asynchronous operations and Axum for HTTP/WebSocket handling. It integrates with the existing JWT-based authentication system.

## Features

1. **User Authentication**: Only authenticated users can join chat rooms using JWT tokens
2. **Room-based Chat**: Users can join specific chat rooms (default: "general")
3. **Real-time Messaging**: Instant message broadcasting to all users in the same room
4. **Join/Leave Notifications**: Automatic system notifications when users join or leave
5. **Asynchronous Architecture**: Non-blocking operations for high scalability
6. **In-memory Storage**: Efficient user and room management using thread-safe data structures

## Architecture

### Components

1. **ChatState**: Central state management for connected users and rooms
2. **WebSocket Handler**: Authenticates users and manages WebSocket connections
3. **Message Broadcasting**: Distributes messages to all users in a room
4. **Room Management**: Organizes users into chat rooms

### Data Structures

```rust
// In-memory storage for connected users
struct ConnectedUser {
    user_id: Uuid,
    username: String,
    room_name: String,
}

// Chat server state
struct ChatState {
    connected_users: Arc<Mutex<HashMap<Uuid, ConnectedUser>>>,
    rooms: Arc<Mutex<HashMap<String, broadcast::Sender<Message>>>>,
}
```

## API Endpoints

### WebSocket Connection

```
GET /ws
```

**Query Parameters:**
- `token` (required): JWT access token for authentication
- `room` (optional): Room name to join (default: "general")

**Headers:**
- `Authorization: Bearer <token>`

**Example:**
```
GET /ws?token=eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...&room=general
Authorization: Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...
```

## Message Formats

### User Messages

```json
{
  "user_id": "string",
  "username": "string",
  "message": "string",
  "timestamp": "u64"
}
```

### System Messages

```json
{
  "message": "string",
  "timestamp": "u64"
}
```

System messages are prefixed with "system:" to distinguish them from user messages.

## Implementation Details

### Authentication Flow

1. User obtains JWT token through login endpoint (`/auth/login`)
2. User connects to WebSocket with token in both query parameter and Authorization header
3. Server validates token using existing JWT utility functions
4. If valid, user is added to the requested room
5. If invalid or expired, connection is rejected

### Connection Handling

1. WebSocket connection is upgraded after authentication
2. User is added to in-memory storage
3. "User joined" notification is broadcast to room
4. Separate tasks handle:
   - Receiving messages from the client
   - Sending messages to the client from the room
5. On disconnect:
   - User is removed from in-memory storage
   - "User left" notification is broadcast to room

### Room Management

1. Rooms are created on-demand when the first user joins
2. Each room has a broadcast channel for message distribution
3. Users can join any room by name
4. Default room is "general" if none specified

## Testing

### Unit Tests

Run unit tests with:
```bash
cargo test
```

Tests include:
- Chat state management (adding/removing users)
- Room creation and management

### Manual Testing

1. Start the server:
```bash
cargo run
```

2. Register a user:
```bash
curl -X POST http://localhost:8080/auth/register \
  -H "Content-Type: application/json" \
  -d '{
    "full_name": "Test User",
    "email": "test@example.com",
    "age": 30,
    "password": "password123",
    "date_of_birth": "1993-05-15T00:00:00Z",
    "gender": "male",
    "phone_number": "+1234567890"
  }'
```

3. Login to get JWT token:
```bash
curl -X POST http://localhost:8080/auth/login \
  -H "Content-Type: application/json" \
  -d '{
    "email": "test@example.com",
    "password": "password123"
  }'
```

4. Connect to chat using the token from step 3:
```bash
# Use a WebSocket client to connect to:
# ws://localhost:8080/ws?token=<your_token_here>
```

## Dependencies

- `axum` with "ws" and "headers" features
- `tokio` with full features
- `serde` for serialization
- `uuid` for user identification
- `futures` for async utilities
- Existing JWT utilities from the authentication module

## Performance Considerations

- Uses broadcast channels for efficient message distribution
- Arc<Mutex<...>> for thread-safe shared state
- Non-blocking operations throughout
- Minimal memory allocation in hot paths

## Security

- JWT token validation for all connections
- No anonymous access to chat
- Token expiration handled by existing auth system
- User identity verified before room entry

## Scalability

- Independent tasks per user connection
- Room-based message broadcasting reduces load
- In-memory storage with efficient data structures
- Tokio runtime for optimal async performance