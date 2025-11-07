# RUST-AXUM-PROJECT Authentication API

A professional Rust web application implementing a JWT-based authentication API with a clean architecture structure.

## Project Structure

```
src/
├── main.rs
├── lib.rs
│
├── config/                         # App-wide configuration
│   ├── mod.rs
│   ├── database.rs                # Database connection setup
│   └── environment.rs             # Environment variable management
│
├── infrastructure/                 # Low-level infrastructure
│   ├── mod.rs
│   ├── db/
│   │   ├── mod.rs
│   │   └── connection.rs         # Database connection initialization
│
├── modules/                        # Business domain modules
│   ├── mod.rs
│   └── auth/
│       ├── mod.rs
│       ├── dto/
│       │   ├── mod.rs
│       │   └── auth_dto.rs       # Data Transfer Objects for API
│       ├── entities/
│       │   ├── mod.rs
│       │   └── user.rs           # Domain entities/models
│       ├── repositories/
│       │   ├── mod.rs
│       │   └── auth_repository.rs # Auth data access layer
│       ├── service/
│       │   ├── mod.rs
│       │   └── auth_service.rs   # Business logic layer
│       └── utils/
│           ├── mod.rs
│           └── jwt.rs            # JWT token utilities
│
├── routes/                         # API route configuration
│   ├── mod.rs
│   └── auth_routes.rs            # Auth module routes
│
└── utils/                          # Utility functions
    ├── mod.rs
    └── logger.rs                 # Application logging setup
```

## Features

- JWT-based authentication (access and refresh tokens)
- User registration and login
- User profile retrieval
- Password change functionality
- Refresh token mechanism
- PostgreSQL database with SQLx
- Clean architecture with separation of concerns
- Environment-based configuration
- Structured error handling
- Standardized API responses
- Swagger/OpenAPI documentation

## Prerequisites

- Rust (latest stable version)
- PostgreSQL database
- Environment variables configured (see .env.example)

## Setup

1. Clone the repository
2. Create a `.env` file with your database configuration:
   ```bash
   cp .env.example .env
   ```
   Then edit the `.env` file with your actual database credentials:
   ```env
   DATABASE_URL=postgresql://username:password@localhost:5432/database_name
   ```
3. Build and run the application:
   ```bash
   cargo run
   ```

## API Endpoints

- `POST /auth/register` - Register a new user
- `POST /auth/login` - Login and get JWT tokens
- `POST /auth/me` - Get current user profile
- `POST /auth/refresh-token` - Refresh access token
- `POST /auth/change-password` - Change user password
- `GET /swagger-ui` - API documentation

## Environment Variables

- `DATABASE_URL` - PostgreSQL connection string
- `PORT` - Server port (default: 3005)
- `AUTH_JWT_SECRET` - Secret key for JWT access tokens
- `AUTH_JWT_TOKEN_EXPIRES_IN` - Access token expiration time
- `AUTH_REFRESH_SECRET` - Secret key for JWT refresh tokens
- `AUTH_REFRESH_TOKEN_EXPIRES_IN` - Refresh token expiration time
- `AUTH_FORGOT_TOKEN_EXPIRES_IN` - Forgot password token expiration time
- `AUTH_CONFIRM_EMAIL_TOKEN_EXPIRES_IN` - Email confirmation token expiration time

## Development

- Run tests: `cargo test`
- Check code: `cargo check`
- Format code: `cargo fmt`
- Lint code: `cargo clippy`

## Architecture Layers

1. **Routes Layer**: HTTP request handlers that parse requests and format responses
2. **Service Layer**: Business logic implementation
3. **Repository Layer**: Data access layer that interacts with the database
4. **Entity Layer**: Domain models that represent business entities
5. **DTO Layer**: Data Transfer Objects for API contracts
6. **Utils Layer**: Utility functions (JWT handling)
7. **Infrastructure Layer**: Low-level technical implementations
8. **Configuration Layer**: Environment and application configuration

## JWT Token Management

- **Access Tokens**: Short-lived tokens for authentication (default 24 hours)
- **Refresh Tokens**: Long-lived tokens for session extension (default 365 days)

## Database Schema

The application works with a simple users table with the following columns:
- `id` (UUID) - Primary key
- `name` (VARCHAR) - User's full name
- `email` (VARCHAR) - User's email address
- `created_at` (TIMESTAMP) - Record creation timestamp
- `updated_at` (TIMESTAMP) - Record update timestamp

Note: The current implementation works with an existing database schema. For a complete implementation with all user fields, you would need to update the database schema accordingly.

## Contributing

1. Fork the repository
2. Create a feature branch
3. Commit your changes
4. Push to the branch
5. Create a pull request

## License

This project is licensed under the MIT License.