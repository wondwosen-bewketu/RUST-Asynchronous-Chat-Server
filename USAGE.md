# Asynchronous Chat Server Usage

## Overview

The asynchronous chat server is now fully integrated into your Rust Axum project. This document provides instructions on how to use and interact with the chat server.

## Starting the Server

To start the server with chat functionality:

```bash
cargo run
```

The server will start on the port specified in your environment configuration (default: 3005).

## Connecting to the Chat Server

### WebSocket Endpoint

```
GET /ws
```

### Authentication

The chat server requires JWT authentication. Users can provide the token in either:

1. Query parameter: `?token=YOUR_JWT_TOKEN`
2. Authorization header: `Authorization: Bearer YOUR_JWT_TOKEN`
3. Both (recommended for compatibility)

Example connections:
```bash
# Using query parameter only
ws://localhost:3005/ws?token=YOUR_JWT_TOKEN

# Using Authorization header only
ws://localhost:3005/ws
Header: Authorization: Bearer YOUR_JWT_TOKEN

# Using both (most compatible)
ws://localhost:3005/ws?token=YOUR_JWT_TOKEN
Header: Authorization: Bearer YOUR_JWT_TOKEN
```

### Room Selection

Users can join specific rooms by adding a `room` parameter:
```
/ws?token=YOUR_JWT_TOKEN&room=room_name
```

If no room is specified, users will join the default "general" room.

## Message Formats

### Sending Messages

Simply send text messages through the WebSocket connection:
```
Hello everyone!
```

### Receiving Messages

Messages are received in two formats:

1. **User Messages**:
```json
{
  "user_id": "user-uuid",
  "username": "User_xxxxxxxx",
  "message": "Hello everyone!",
  "timestamp": 1234567890
}
```

2. **System Messages** (prefixed with "system:"):
```json
system:{"message":"User_xxxxxxxx has joined the chat.","timestamp":1234567890}
```

## Testing the Chat Server

### Using the Test Client

A test client binary is provided for easy testing:

```bash
cargo run --bin chat_client http://localhost:3005 YOUR_JWT_TOKEN [ROOM_NAME]
```

### Manual Testing with wscat

1. Install wscat:
```bash
npm install -g wscat
```

2. Connect to the server using either method:
```bash
# Method 1: Using query parameter
wscat -c "ws://localhost:3005/ws?token=YOUR_JWT_TOKEN"

# Method 2: Using Authorization header
wscat -c "ws://localhost:3005/ws" --header "Authorization: Bearer YOUR_JWT_TOKEN"
```

3. Send messages by typing them in the terminal.

## Integration with Existing Authentication

The chat server seamlessly integrates with your existing JWT-based authentication system:

- Uses the same [JwtUtil](file:///Users/etmsoftware/Desktop/Rust/rust-axum-project/src/modules/auth/utils/jwt.rs#L22-L22) for token validation
- Works with the same token format
- Respects token expiration times

## Scalability and Performance

The chat server is designed for high performance:

- Asynchronous architecture using Tokio
- Broadcast channels for efficient message distribution
- Thread-safe in-memory storage with Arc<Mutex<...>>
- Non-blocking operations throughout

## Features Summary

✅ Only authenticated users can join chat
💬 Users join "rooms" (default: "general")
🔁 Messages broadcast to all users in the same room
🔔 System notifications for join/leave events
⚙️ Asynchronous message handling powered by Tokio

## Troubleshooting

### Common Issues

1. **Connection refused**: Ensure the server is running on the correct port
2. **401 Unauthorized**: Check that your JWT token is valid and not expired
3. **404 Not Found**: Verify the WebSocket endpoint is `/ws`

### Checking Server Logs

View server logs in the terminal where you started the server for debugging information.

## Extending the Chat Server

### Adding New Features

The chat server is designed to be extensible. You can add features by:

1. Modifying the [ChatState](file:///Users/etmsoftware/Desktop/Rust/rust-axum-project/src/modules/chat/server.rs#L30-L33) struct to store additional information
2. Adding new message types to the serialization structures
3. Extending the [handle_socket](file:///Users/etmsoftware/Desktop/Rust/rust-axum-project/src/modules/chat/server.rs#L119-L192) function to handle new message types

### Custom Room Management

The current implementation creates rooms on-demand. You can extend this by:

1. Pre-defining rooms
2. Adding room administration features
3. Implementing room persistence

## Security Considerations

- All connections require valid JWT tokens
- Tokens are validated using the same security measures as the REST API
- User identities are verified before allowing chat participation
- Message content is not filtered (consider adding content filtering if needed)