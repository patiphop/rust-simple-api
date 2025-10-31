# Rust Simple API

A simple REST API built with Rust and MongoDB that demonstrates basic CRUD operations for user management.

## Description

This project is a minimal web API implementation using:
- **Rust** - A systems programming language focused on performance and safety
- **Warp** - A lightweight web framework for Rust
- **MongoDB** - A NoSQL document database
- **Tokio** - Asynchronous runtime for Rust

The API provides endpoints for managing users with basic CRUD (Create, Read, Update, Delete) operations.

## Technology Stack

### Why Warp?

Warp was selected as the web framework for this project after careful consideration of several alternatives. The decision was based on the following factors:

**Performance & Efficiency**
- Warp is built on top of Hyper and leverages Rust's zero-cost abstractions, providing excellent performance characteristics
- The framework is designed with a focus on minimal overhead and efficient request handling
- Its filter-based architecture allows for precise control over request processing pipeline

**Type-Safe Routing**
- Warp's composable filter system provides compile-time guarantees for route definitions
- Type safety extends to path parameters, query parameters, and request body validation
- This reduces runtime errors and improves overall code reliability

**Ergonomics & Developer Experience**
- The filter-based approach creates an intuitive, expressive API for defining routes and middleware
- Clean separation of concerns with a functional programming style
- Excellent integration with Tokio's async/await model, making asynchronous code natural to write

**Ecosystem Compatibility**
- Seamless integration with MongoDB through the official MongoDB driver
- Native support for JSON serialization/deserialization with Serde
- Works perfectly with Tokio's asynchronous runtime, which is already used for database operations

**Architecture Alignment**
- Warp's lightweight nature complements the project's goal of being a simple, focused API
- The framework doesn't impose unnecessary abstractions, keeping the codebase approachable
- Its modular design allows for easy extension as the API grows in complexity

For this specific use case of a simple CRUD API with MongoDB, Warp provides the right balance of performance, safety, and developer productivity without unnecessary complexity.

## Features

- RESTful API design
- MongoDB integration for data persistence
- Async/await support with Tokio
- JSON request/response handling
- Error handling and validation
- Health check endpoint
- Seed data functionality
- CORS support
- Test coverage for API endpoints

## Prerequisites

Before running this application, ensure you have the following installed:

- **Rust** (latest stable version) - [Install Rust](https://rustup.rs/)
- **Docker** and **Docker Compose** - [Install Docker](https://docs.docker.com/get-docker/) (recommended for MongoDB)
- **Git** - For version control

**Important:** MongoDB must be running before using the application. The recommended approach is to use Docker.

## Quick Start

### 1. Clone the Repository

```bash
git clone <repository-url>
cd rust-simple-api
```

### 2. Set Up Environment Variables

The project requires a `.env` file for configuration. Create one based on the example below:

```bash
# MongoDB Configuration (Docker setup - recommended)
MONGODB_URI=mongodb://api_user:api_password@localhost:27017/simple_api_db
DATABASE_NAME=simple_api_db

# Server Configuration
PORT=3030
```

**Note:** The `.env` file is gitignored for security reasons. You'll need to create it manually.

### 3. Start MongoDB with Docker (Recommended)

The easiest way to get started is using the provided Docker setup:

```bash
# Start MongoDB using Docker
./docker_setup.sh start

# Check MongoDB status
./docker_setup.sh status
```

This will:
- Automatically stop any native MongoDB service
- Start MongoDB in a Docker container
- Set up the database and user
- Provide connection information

### 4. Build the Application

```bash
cargo build
```

### 5. Run the Application

```bash
# Development mode
cargo run

# Or run the compiled binary
./target/debug/rust-simple-api
```

The server will start on `http://localhost:3030` (or the port specified in your `.env` file).

### 6. Load Seed Data (Manual)

**Important:** Seed data is NOT automatically loaded. You must manually trigger it:

```bash
# Load seed data
./seed_data.sh seed

# Or use the binary directly
./target/debug/rust-simple-api seed
```

### 7. Stop MongoDB (When Done)

```bash
# Stop MongoDB container
./docker_setup.sh stop
```

## Alternative: Native MongoDB Installation

If you prefer to install MongoDB natively instead of using Docker:

**Quick native installation options:**

**macOS (Homebrew):**
```bash
brew tap mongodb/brew
brew install mongodb-community
brew services start mongodb-community
```

**Ubuntu/Debian:**
```bash
wget -qO - https://www.mongodb.org/static/pgp/server-7.0.asc | sudo apt-key add -
echo "deb [ arch=amd64,arm64 ] https://repo.mongodb.org/apt/ubuntu jammy/mongodb-org/7.0 multiverse" | sudo tee /etc/apt/sources.list.d/mongodb-org-7.0.list
sudo apt-get update
sudo apt-get install -y mongodb-org
sudo systemctl start mongod
```

When using native MongoDB, update your `.env` file to use:
```bash
# For native MongoDB installation
MONGODB_URI=mongodb://localhost:27017
```

## API Endpoints

### Health Check
- `GET /health` - Check if the API server is running

### Users
- `GET /users` - Retrieve all users
- `GET /users/{id}` - Retrieve a specific user by ID
- `POST /users` - Create a new user

### API Response Examples

#### Health Check Response
```json
{
  "status": "ok",
  "timestamp": "2023-12-01T10:30:00Z"
}
```

#### Get All Users Response
```json
{
  "users": [
    {
      "_id": "656a1b2c3d4e5f6789012345",
      "name": "John Doe",
      "email": "john@example.com",
      "created_at": "2023-12-01T10:30:00Z"
    }
  ]
}
```

#### Create User Response
```json
{
  "success": true,
  "user": {
    "_id": "656a1b2c3d4e5f6789012346",
    "name": "Jane Smith",
    "email": "jane@example.com",
    "created_at": "2023-12-01T10:35:00Z"
  }
}
```

### Error Responses

#### User Not Found
```json
{
  "error": "User not found",
  "code": 404
}
```

#### Invalid ID Format
```json
{
  "error": "Invalid ID format",
  "code": 400
}
```

#### Validation Error
```json
{
  "error": "Name and email are required",
  "code": 400
}
```

### Quick API Testing Examples

Here are some curl examples for quick API testing:

```bash
# Health check
curl -X GET http://localhost:3030/health

# Get all users
curl -X GET http://localhost:3030/users

# Get user by ID
curl -X GET http://localhost:3030/users/6904df356f45a9f3f7495fb1

# Create new user
curl -X POST \
  -H "Content-Type: application/json" \
  -d '{"name":"Test User","email":"test@example.com"}' \
  http://localhost:3030/users

# Error handling examples
curl -X GET http://localhost:3030/users/507f1f77bcf86cd799439011  # User not found
curl -X GET http://localhost:3030/users/invalid-id  # Invalid ID format
```

## Testing

This project includes unit and integration tests for API endpoints.

### Running Tests

```bash
# Run all tests (unit + integration)
cargo test

# Run only unit tests
cargo test --lib

# Run only integration tests
cargo test --test integration_tests

# Run tests with output
cargo test -- --nocapture

# Run tests in release mode
cargo test --release

# Additional test options
cargo test -- --format pretty                    # Pretty test output
cargo test --no-fail-fast                        # Continue on test failures
cargo test -- --test-threads=1                   # Run tests sequentially
cargo test models::user                          # Run tests for specific module
cargo test --test integration_tests test_health  # Run specific test
```

### Test Coverage

The project includes tests for:
- Unit tests for core functionality and data models
- Integration tests for API endpoints
- Error handling scenarios

## Project Structure

```
rust-simple-api/
├── src/
│   ├── db/           # Database connection and operations
│   ├── handlers/     # HTTP request handlers
│   ├── models/       # Data models and structs
│   └── main.rs       # Application entry point
├── tests/               # Integration tests and test utilities
│   ├── integration_tests.rs  # End-to-end API tests
│   ├── test_config.rs      # Test configuration and utilities
│   └── mod.rs            # Test module declarations
├── docker-compose.yml    # Docker Compose configuration for MongoDB
├── docker_setup.sh       # Docker MongoDB management script
├── mongo-init.js          # MongoDB initialization script
├── seed_data.sh          # Database seeding script
├── test_api.sh           # API testing script
└── .env.example          # Environment variables example
```

## Configuration

The application can be configured using environment variables in the `.env` file:

- `MONGODB_URI` - MongoDB connection string (Docker: `mongodb://api_user:api_password@localhost:27017/simple_api_db`)
- `DATABASE_NAME` - Name of the MongoDB database (default: `simple_api_db`)
- `PORT` - Server port (default: `3030`)

### Docker Configuration

When using Docker (recommended), the MongoDB service is configured with:
- **Container Name:** `rust-simple-api-mongodb`
- **Port:** `27017` (mapped to host)
- **Database:** `simple_api_db`
- **Username:** `api_user`
- **Password:** `api_password`

Use the provided `docker_setup.sh` script to manage the MongoDB container.

### Resetting Database Data

To reset the database and reload the seed data:

```bash
# Stop and remove the current MongoDB container
docker-compose down -v

# Start a fresh container
docker-compose up -d

# Manually seed the data
./seed_data.sh seed
```

This will completely remove all data and start fresh with the sample users.

## Database Operations

The application includes several database management commands for manual data management:

```bash
# Seed the database with sample data (manual seeding)
./target/debug/rust-simple-api seed

# Clear all users from the database
./target/debug/rust-simple-api seed clear

# Count current users in the database
./target/debug/rust-simple-api seed count

# Clear and reseed the database
./target/debug/rust-simple-api seed reseed
```

Or use the convenient shell script:

```bash
./seed_data.sh [seed|clear|count|reseed]
```

**Note:** Seed data must be manually loaded using these commands.

## Development

### Running in Development Mode

```bash
cargo run
```

### Building for Production

```bash
cargo build --release
```

### Running Tests

```bash
cargo test
```

## Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

### Testing Requirements for Contributors

- All new features should include appropriate tests
- API changes should include integration tests
- Follow existing testing patterns in the codebase

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Support

For Docker-specific issues, use the provided `docker_setup.sh help` command.

For other issues or questions, please open an issue in the repository.

## Troubleshooting

### Common Issues

**MongoDB Connection Failed**
- Ensure MongoDB is running: `./docker_setup.sh status`
- Check your `.env` file configuration
- Verify the MongoDB URI matches your setup

**Port Already in Use**
- Check if another service is using port 3030: `lsof -i :3030`
- Change the PORT in your `.env` file to a different value

**Docker Issues**
- Ensure Docker is running: `docker --version`
- Restart Docker daemon if needed
- Try `docker system prune` to clean up unused containers

**Build Failures**
- Update Rust: `rustup update`
- Clear cargo cache: `cargo clean`
- Check for missing dependencies: `cargo check`

**Seed Data Not Loading**
- Ensure the application is built: `cargo build`
- Check MongoDB connection: `./docker_setup.sh status`
- Run seed command manually: `./seed_data.sh seed`

### Getting Help

1. Check the troubleshooting steps above
2. Run `./docker_setup.sh help` for Docker-related commands
3. Check application logs for error messages
4. Open an issue in the repository with details about your problem

## Future Improvements

Based on the current implementation, here are potential improvements for future development:

### High Priority
- **Enhanced Input Validation**: Add email format validation and name length limits
- **Complete CRUD Operations**: Implement PUT/PATCH for user updates and DELETE for user removal
- **Pagination**: Add pagination for the GET /users endpoint to handle large datasets

### Medium Priority
- **Authentication & Authorization**: Implement JWT-based authentication for secure API access
- **Error Response Standardization**: Create unified error response structures for better client handling
- **API Documentation**: Generate OpenAPI/Swagger specification for interactive API documentation

### Low Priority
- **Rate Limiting**: Implement request rate limiting to prevent API abuse
- **Search & Filtering**: Add search capabilities and filtering options for users
- **Performance Optimization**: Add database indexes for improved query performance