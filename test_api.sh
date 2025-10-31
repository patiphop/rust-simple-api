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