# Chat Server Testing Guide

This guide provides step-by-step instructions for testing the asynchronous chat server implementation.

## Prerequisites

1. Rust and Cargo installed
2. PostgreSQL database running
3. Environment variables configured in `.env` file

## Step 1: Start the Server

1. Open a terminal
2. Navigate to the project directory:
   ```bash
   cd /Users/etmsoftware/Desktop/Rust/rust-axum-project
   ```

3. Start the server:
   ```bash
   cargo run
   ```

4. Wait for the server to start. You should see output similar to:
   ```
   Listening on 0.0.0.0:3005
   API Documentation available at: http://0.0.0.0:3005/swagger-ui/
   Health check endpoint: http://0.0.0.0:3005/health
   ```

**Note**: The port may be different based on your `.env` configuration. Check the "Listening on" message to see the actual port.

## Step 2: Register a Test User

1. Open another terminal window
2. Register a new user:
   ```bash
   curl -X POST http://localhost:3005/auth/register \
     -H "Content-Type: application/json" \
     -d '{
       "full_name": "Alice Smith",
       "email": "alice@example.com",
       "age": 28,
       "password": "password123",
       "date_of_birth": "1995-03-15T00:00:00Z",
       "gender": "female",
       "phone_number": "+1234567890"
     }'
   ```

3. You should receive a success response:
   ```json
   {
     "success": true,
     "message": "User registered successfully",
     "user": {
       "id": "user-uuid-here",
       "name": "Alice Smith",
       "email": "alice@example.com"
     }
   }
   ```

## Step 3: Login to Get JWT Token

1. Login with the registered user:
   ```bash
   curl -X POST http://localhost:3005/auth/login \
     -H "Content-Type: application/json" \
     -d '{
       "email": "alice@example.com",
       "password": "password123"
     }'
   ```

2. You should receive a response with access and refresh tokens:
   ```json
   {
     "success": true,
     "message": "Login successful",
     "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
     "refresh_token": "refresh-token-here",
     "expires_in": 3600
   }
   ```

3. Copy the `token` value - you'll need it for WebSocket authentication.

## Step 4: Connect to Chat Server

There are three ways to test the WebSocket connection:

### Option A: Using a WebSocket Client with Query Parameter (Recommended)

1. Install a WebSocket testing tool like [wscat](https://www.npmjs.com/package/wscat):
   ```bash
   npm install -g wscat
   ```

2. Connect to the chat server using your JWT token (replace 3005 with your actual port):
   ```bash
   wscat -c "ws://localhost:3005/ws?token=YOUR_JWT_TOKEN_HERE"
   ```

3. Replace `YOUR_JWT_TOKEN_HERE` with the actual token from Step 3.

4. You should see a system message indicating you've joined the chat:
   ```json
   system:{"message":"User_xxxxxxxx has joined the chat.","timestamp":1234567890}
   ```

### Option B: Using a WebSocket Client with Authorization Header

1. Connect using the Authorization header:
   ```bash
   wscat -c "ws://localhost:3005/ws" --header "Authorization: Bearer YOUR_JWT_TOKEN_HERE"
   ```

### Option C: Using the Test Client Binary

1. In a new terminal, run the chat client:
   ```bash
   cargo run --bin chat_client http://localhost:3005 YOUR_JWT_TOKEN_HERE
   ```

2. Replace `YOUR_JWT_TOKEN_HERE` with the actual token from Step 3.

## Step 5: Test Chat Functionality

1. After connecting, try sending a message:
   ```
   Hello everyone!
   ```

2. You should see your message echoed back:
   ```json
   {"user_id":"user-id-here","username":"User_xxxxxxxx","message":"Hello everyone!","timestamp":1234567891}
   ```

3. Connect with another user (repeat Steps 2-4 with a different email) to see real-time messaging between users.

## Step 6: Test Room Functionality

1. Connect to a specific room by adding the room parameter:
   ```bash
   wscat -c "ws://localhost:3005/ws?token=YOUR_JWT_TOKEN_HERE&room=room1"
   ```

2. Messages sent in this room will only be visible to users in the same room.

## Step 7: Test User Leave Notification

1. Close the WebSocket connection (Ctrl+C in wscat)
2. Other connected users should see a leave notification:
   ```json
   system:{"message":"User_xxxxxxxx has left the chat.","timestamp":1234567892}
   ```

## Troubleshooting

### Common Issues

1. **Connection refused**: Make sure the server is running on the correct port
2. **Authentication failed**: Verify your JWT token is valid and not expired
3. **404 Not Found**: Check that the WebSocket endpoint is `/ws`
4. **Port already in use**: Kill the existing process or change the port in `.env`

### Checking Server Logs

Look at the terminal where you started the server for any error messages or debugging information.

### Verifying Environment Variables

Make sure your `.env` file contains the required configuration:
```
PORT=3005
DATABASE_URL=your_database_url
AUTH_JWT_SECRET=your_secret_key
AUTH_REFRESH_SECRET=your_refresh_secret
AUTH_JWT_TOKEN_EXPIRES_IN=24hr
AUTH_REFRESH_TOKEN_EXPIRES_IN=365d
```

## Running Unit Tests

To verify the implementation works correctly:

```bash
cargo test
```

This will run all unit tests, including chat state management tests.