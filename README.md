# Rust Simple API

A simple REST API built with Rust and MongoDB for user management.

## Technology Stack

- Rust with Warp web framework
- MongoDB for data persistence
- Tokio for async runtime

## Quick Start

1. Clone the repository:
```bash
git clone <repository-url>
cd rust-simple-api
```

2. Set up environment variables (create `.env` file):
```bash
MONGODB_URI=mongodb://api_user:api_password@localhost:27017/simple_api_db
DATABASE_NAME=simple_api_db
PORT=3030
```

3. Start MongoDB with Docker:
```bash
./docker_setup.sh start
```

4. Build and run the application:
```bash
cargo build
cargo run
```

5. Load seed data (optional):
```bash
./seed_data.sh seed
```

## API Endpoints

- `GET /health` - Health check
- `GET /users` - Get all users
- `GET /users/{id}` - Get user by ID
- `POST /users` - Create new user

## API Examples

### Health Check
```bash
curl -X GET http://localhost:3030/health
```

### Get Users
```bash
curl -X GET http://localhost:3030/users
```

### Create User
```bash
curl -X POST \
  -H "Content-Type: application/json" \
  -d '{"name":"Test User","email":"test@example.com"}' \
  http://localhost:3030/users
```

## Testing

```bash
# Run all tests
cargo test

# Run integration tests only
cargo test --test integration_tests
```

## Project Structure

```
rust-simple-api/
├── src/
│   ├── db/           # Database operations
│   ├── handlers/     # HTTP request handlers
│   ├── models/       # Data models
│   └── main.rs       # Application entry point
├── tests/            # Integration tests
├── docker-compose.yml    # Docker configuration
├── docker_setup.sh       # MongoDB management script
└── seed_data.sh          # Database seeding script
```

## Database Operations

```bash
# Seed database
./seed_data.sh seed

# Clear all users
./seed_data.sh clear

# Count users
./seed_data.sh count

# Reset and reseed
./seed_data.sh reseed
```

## Stop MongoDB

```bash
./docker_setup.sh stop
