# Rust Simple API

A simple REST API built with Rust and MongoDB 8.2 for user management.

## Technology Stack

- Rust with Warp web framework
- MongoDB for data persistence
- Tokio for async runtime

## Quick Start

1. Clone the repository:
```bash
git clone https://github.com/patiphop/rust-simple-api
cd rust-simple-api
```

2. Set up environment variables (create `.env` file):
```bash
MONGODB_URI=mongodb://admin:admin@localhost:27017/simple_api_db
DATABASE_NAME=simple_api_db
PORT=3030
```

3. Initialize MongoDB with Docker:
```bash
# Start MongoDB container with default credentials
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

## Test Data Cleanup

The integration tests include automatic cleanup functionality that tracks created users and logs cleanup operations. However, due to runtime constraints, the actual cleanup is disabled to avoid conflicts.

### Automatic Cleanup (Logging Only)
- Tests automatically track created users using a `TestGuard` pattern
- Cleanup operations are logged when tests complete
- Test data may remain in the database after test runs

### Manual Cleanup
To clean up test data manually, run:

```bash
# Clean up test users from database
./cleanup_test_data.sh
```

This script removes users with test-specific patterns:
- Users with `@test.com` email addresses
- Users with "Test", "Integration", "Concurrent", or "Database" in their names

### Database Reset Options
For complete database management, use the seed data script:

```bash
# Clear all users
./seed_data.sh clear

# Count current users
./seed_data.sh count

# Reset and reseed with fresh data
./seed_data.sh reseed
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
