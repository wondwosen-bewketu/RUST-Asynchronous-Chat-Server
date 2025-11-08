# Multithreaded File Indexer Documentation

## Overview

The Multithreaded File Indexer is a high-performance file scanning module that integrates with your existing Axum + Auth backend. It provides both CLI and API access for detecting duplicate files in directories using SHA256 hashing and multithreaded processing.

## Features

1. **User Authentication Integration**: Works with existing JWT-based authentication system
2. **Multithreaded Processing**: Uses multiple threads for faster file scanning
3. **SHA256 Hashing**: Reliable file content hashing for accurate duplicate detection
4. **CLI + API Support**: Available as both command-line tool and HTTP API endpoint
5. **Asynchronous Operation**: Non-blocking operations using Tokio
6. **Security**: Path validation and user authentication for API access

## Architecture

### Components

1. **Scanner Module** (`src/modules/file_indexer/scanner.rs`)
   - File system traversal and collection
   - SHA256 hashing implementation
   - Duplicate detection logic
   - Multithreaded processing

2. **Service Module** (`src/modules/file_indexer/service.rs`)
   - Integration with Axum backend
   - Asynchronous task management
   - CLI and API interface

3. **Routes Module** (`src/routes/file_routes.rs`)
   - HTTP endpoint for authenticated file scanning
   - Request/response handling

4. **CLI Tool** (`src/bin/file_indexer_cli.rs`)
   - Standalone command-line interface
   - Direct file system access

## API Endpoints

### Scan Files for Duplicates

```
POST /api/files/scan
```

**Headers:**
- `Authorization: Bearer <jwt_token>`

**Request Body:**
```json
{
  "directory": "/path/to/directory"
}
```

**Response:**
```json
{
  "duplicates": {
    "abc123...": ["file1.png", "file2.png"],
    "def456...": ["doc1.pdf", "doc2.pdf"]
  },
  "total_files": 124,
  "time_taken": "2.3s"
}
```

## CLI Usage

```bash
# Scan a directory with default settings
cargo run --bin file_indexer_cli /path/to/directory

# Scan a directory with custom thread count
cargo run --bin file_indexer_cli /path/to/directory 8
```

## Implementation Details

### Authentication Flow

1. User authenticates through existing `/auth/login` endpoint
2. User receives JWT token
3. User calls `/api/files/scan` with token in Authorization header
4. Backend validates token and authorizes request
5. Backend starts file scanning in background task

### Multithreaded Processing

1. Directory is scanned recursively to collect all files
2. Files are divided into chunks based on thread count
3. Each chunk is processed by a separate worker thread
4. Worker threads calculate SHA256 hashes for each file
5. Results are collected in a shared HashMap (protected by Mutex)
6. Duplicates are identified by grouping files with identical hashes

### Security Considerations

1. Only authenticated users can access the API endpoint
2. Directory paths are validated to prevent directory traversal attacks
3. File access is limited to the specified directory and subdirectories

## Performance

- Uses multiple threads for parallel file processing
- Reads files in chunks to handle large files efficiently
- Non-blocking operations using Tokio async tasks
- Memory-efficient hash storage using HashMap

## Integration with Existing System

The file indexer seamlessly integrates with your existing Axum + Auth backend:

- Uses the same JWT validation as other endpoints
- Follows the same error handling patterns
- Compatible with existing database connection pooling
- Maintains consistent API response formats

## Example Usage

### API Request
```bash
# Login to get token
curl -X POST http://localhost:3005/auth/login \
  -H "Content-Type: application/json" \
  -d '{"email": "user@example.com", "password": "password"}'

# Use token to scan files
curl -X POST http://localhost:3005/api/files/scan \
  -H "Authorization: Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9..." \
  -H "Content-Type: application/json" \
  -d '{"directory": "/home/uploads"}'
```

### CLI Usage
```bash
# Scan directory with 4 threads
cargo run --bin file_indexer_cli /home/uploads 4
```

## Response Format

```json
{
  "duplicates": {
    "a1b2c3d4e5f6...": [
      "/home/uploads/image1.jpg",
      "/home/uploads/backup/image1_copy.jpg"
    ],
    "f6e5d4c3b2a1...": [
      "/home/uploads/document.pdf",
      "/home/uploads/archive/document_backup.pdf"
    ]
  },
  "total_files": 150,
  "time_taken": "1.8s"
}
```

## Error Handling

The system provides comprehensive error handling:

- Invalid authentication tokens return 401 Unauthorized
- Invalid directory paths return 400 Bad Request
- File system errors return 500 Internal Server Error
- Thread execution errors are properly propagated

## Extensibility

The modular design allows for easy extension:

- Add support for different hash algorithms
- Implement file size or type filtering
- Add progress reporting for large scans
- Integrate with cloud storage providers