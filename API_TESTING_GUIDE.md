# Rust Simple API Testing Guide

This document provides comprehensive testing instructions for the Rust Simple API application with MongoDB integration.

## Table of Contents

1. [Environment Setup](#environment-setup)
2. [Building the Application](#building-the-application)
3. [Database Operations](#database-operations)
4. [Starting the Server](#starting-the-server)
5. [API Endpoint Testing](#api-endpoint-testing)
6. [Error Handling Tests](#error-handling-tests)
7. [Test Script](#test-script)

## Environment Setup

### Prerequisites

- Rust (latest stable version)
- MongoDB (running on localhost:27017)
- Git

### Environment Variables

The application uses the following environment variables (defined in `.env`):

```bash
# MongoDB Configuration
MONGODB_URI=mongodb://localhost:27017
DATABASE_NAME=simple_api_db

# Server Configuration
PORT=3030
```

## Building the Application

### Check Compilation

```bash
cargo check
```

### Build the Application

```bash
# Debug build
cargo build

# Release build (for production)
cargo build --release
```

## Database Operations

### Using the Seed Script

The project includes a convenient shell script for database operations:

```bash
# Make the script executable
chmod +x seed_data.sh

# Seed the database with mock data
./seed_data.sh seed

# Clear all users from the database
./seed_data.sh clear

# Count current users in the database
./seed_data.sh count

# Clear and reseed the database
./seed_data.sh reseed

# Show help
./seed_data.sh help
```

### Using the Binary Directly

```bash
# Using debug build
./target/debug/rust-simple-api seed [command]

# Using release build
./target/release/rust-simple-api seed [command]

# Available commands:
# seed    - Seed the database with mock user data (default)
# clear   - Clear all users from the database
# count   - Show the current number of users in the database
# reseed  - Clear existing data and insert fresh mock data
```

## Starting the Server

### Development Mode

```bash
./target/debug/rust-simple-api
```

### Production Mode

```bash
./target/release/rust-simple-api
```

### Background Mode

```bash
./target/debug/rust-simple-api &
```

The server will start on port 3030 (or the port specified in the `PORT` environment variable).

## API Endpoint Testing

### Base URL

```
http://localhost:3030
```

### 1. Health Check Endpoint

**Endpoint:** `GET /health`

**Description:** Check if the API server is running.

```bash
curl -X GET http://localhost:3030/health
```

**Expected Response:**
```json
{
  "status": "ok",
  "timestamp": "2025-10-31T16:09:36.507388Z",
  "version": "1.0.0"
}
```

### 2. Get All Users

**Endpoint:** `GET /users`

**Description:** Retrieve all users from the database.

```bash
curl -X GET http://localhost:3030/users
```

**Expected Response:**
```json
[
  {
    "id": "6904df356f45a9f3f7495fb1",
    "name": "Alice Johnson",
    "email": "alice.johnson@example.com",
    "created_at": "2025-10-31T16:09:25.862422+00:00"
  },
  {
    "id": "6904df356f45a9f3f7495fb2",
    "name": "Bob Smith",
    "email": "bob.smith@example.com",
    "created_at": "2025-10-31T16:09:25.862429+00:00"
  }
]
```

### 3. Get User by ID

**Endpoint:** `GET /users/{id}`

**Description:** Retrieve a specific user by their ID.

```bash
curl -X GET http://localhost:3030/users/6904df356f45a9f3f7495fb1
```

**Expected Response:**
```json
{
  "id": "6904df356f45a9f3f7495fb1",
  "name": "Alice Johnson",
  "email": "alice.johnson@example.com",
  "created_at": "2025-10-31T16:09:25.862422+00:00"
}
```

### 4. Create New User

**Endpoint:** `POST /users`

**Description:** Create a new user in the database.

```bash
curl -X POST \
  -H "Content-Type: application/json" \
  -d '{"name":"Test User","email":"test@example.com"}' \
  http://localhost:3030/users
```

**Expected Response:**
```json
{
  "id": "6904df4506fe72636bd3d29b",
  "name": "Test User",
  "email": "test@example.com",
  "created_at": "2025-10-31T16:09:41.820769+00:00"
}
```

## Error Handling Tests

### 1. User Not Found

**Test:** Request a non-existent user ID

```bash
curl -X GET http://localhost:3030/users/507f1f77bcf86cd799439011
```

**Expected Response (404):**
```json
{
  "error": "not_found",
  "message": "User not found"
}
```

### 2. Invalid ID Format

**Test:** Request with invalid ObjectId format

```bash
curl -X GET http://localhost:3030/users/invalid-id
```

**Expected Response (400):**
```json
{
  "error": "invalid_id",
  "message": "Invalid user ID format"
}
```

### 3. Validation Error - Empty Name

**Test:** Create user with empty name

```bash
curl -X POST \
  -H "Content-Type: application/json" \
  -d '{"name":"","email":"test@example.com"}' \
  http://localhost:3030/users
```

**Expected Response (400):**
```json
{
  "error": "validation_error",
  "message": "Name is required"
}
```

### 4. Validation Error - Empty Email

**Test:** Create user with empty email

```bash
curl -X POST \
  -H "Content-Type: application/json" \
  -d '{"name":"Test User","email":""}' \
  http://localhost:3030/users
```

**Expected Response (400):**
```json
{
  "error": "validation_error",
  "message": "Email is required"
}
```

### 5. Malformed JSON

**Test:** Send malformed JSON

```bash
curl -X POST \
  -H "Content-Type: application/json" \
  -d '{"name":"Test User","email":}' \
  http://localhost:3030/users
```

**Expected Response (400):**
```
Request body deserialize error: expected value at line 1 column 29
```

### 6. Non-existent Endpoint

**Test:** Request a non-existent endpoint

```bash
curl -X GET http://localhost:3030/nonexistent
```

**Expected Response (404):**
```
(empty response with 404 status code)
```

## Test Script

### Automated Testing Script

Create a file named `test_api.sh` with the following content:

```bash
#!/bin/bash

# API Testing Script for Rust Simple API
# Usage: ./test_api.sh

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Function to print colored output
print_status() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

print_test() {
    echo -e "${YELLOW}[TEST]${NC} $1"
}

# Base URL
BASE_URL="http://localhost:3030"

# Check if server is running
print_status "Checking if server is running..."
if ! curl -s "$BASE_URL/health" > /dev/null; then
    print_error "Server is not running. Please start the server first."
    exit 1
fi

print_status "Server is running. Starting API tests..."

# Test 1: Health Check
print_test "Testing health check endpoint..."
response=$(curl -s "$BASE_URL/health")
echo "$response" | jq -e '.status' > /dev/null && print_status "✓ Health check passed" || print_error "✗ Health check failed"

# Test 2: Get all users
print_test "Testing get all users endpoint..."
response=$(curl -s "$BASE_URL/users")
echo "$response" | jq -e '.[0].id' > /dev/null && print_status "✓ Get all users passed" || print_error "✗ Get all users failed"

# Test 3: Get user by ID
print_test "Testing get user by ID endpoint..."
user_id=$(echo "$response" | jq -r '.[0].id')
response=$(curl -s "$BASE_URL/users/$user_id")
echo "$response" | jq -e '.id' > /dev/null && print_status "✓ Get user by ID passed" || print_error "✗ Get user by ID failed"

# Test 4: Create new user
print_test "Testing create user endpoint..."
response=$(curl -s -X POST -H "Content-Type: application/json" -d '{"name":"API Test User","email":"api-test@example.com"}' "$BASE_URL/users")
echo "$response" | jq -e '.id' > /dev/null && print_status "✓ Create user passed" || print_error "✗ Create user failed"

# Test 5: Error handling - User not found
print_test "Testing user not found error handling..."
response=$(curl -s "$BASE_URL/users/507f1f77bcf86cd799439011")
echo "$response" | jq -e '.error' > /dev/null && print_status "✓ User not found error handling passed" || print_error "✗ User not found error handling failed"

# Test 6: Error handling - Invalid ID
print_test "Testing invalid ID error handling..."
response=$(curl -s "$BASE_URL/users/invalid-id")
echo "$response" | jq -e '.error' > /dev/null && print_status "✓ Invalid ID error handling passed" || print_error "✗ Invalid ID error handling failed"

# Test 7: Error handling - Validation
print_test "Testing validation error handling..."
response=$(curl -s -X POST -H "Content-Type: application/json" -d '{"name":"","email":"test@example.com"}' "$BASE_URL/users")
echo "$response" | jq -e '.error' > /dev/null && print_status "✓ Validation error handling passed" || print_error "✗ Validation error handling failed"

print_status "All API tests completed!"
```

Make the script executable and run it:

```bash
chmod +x test_api.sh
./test_api.sh
```

## Additional Notes

### MongoDB Connection Issues

If you encounter MongoDB connection issues:

1. Ensure MongoDB is running: `brew services start mongodb-community` (macOS)
2. Check connection string in `.env` file
3. Verify database name is correct

### Port Conflicts

If port 3030 is already in use:

1. Change the `PORT` environment variable in `.env`
2. Or kill the process using the port: `lsof -ti:3030 | xargs kill`

### CORS

The API is configured to allow requests from any origin (`allow_any_origin()`). For production, consider restricting this to specific origins.

### Logging

The application uses `env_logger` for logging. Set the `RUST_LOG` environment variable to control log levels:

```bash
export RUST_LOG=debug
./target/debug/rust-simple-api