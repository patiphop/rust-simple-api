#!/bin/bash

# Script to populate the MongoDB database with seed data
# Usage: ./seed_data.sh [command]

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

# Check if the binary exists
if [ ! -f "./target/release/rust-simple-api" ] && [ ! -f "./target/debug/rust-simple-api" ]; then
    print_warning "Binary not found. Building the project first..."
    cargo build --release
fi

# Determine which binary to use
if [ -f "./target/release/rust-simple-api" ]; then
    BINARY="./target/release/rust-simple-api"
else
    BINARY="./target/debug/rust-simple-api"
fi

print_status "Using binary: $BINARY"

# Check if MongoDB is running
if ! pgrep -x "mongod" > /dev/null; then
    print_warning "MongoDB might not be running. Please ensure MongoDB is started."
fi

# Handle different commands
case "${1:-seed}" in
    "seed")
        print_status "Seeding database with mock user data..."
        $BINARY seed
        ;;
    "clear")
        print_status "Clearing all users from database..."
        $BINARY seed clear
        ;;
    "count")
        print_status "Getting current user count..."
        $BINARY seed count
        ;;
    "reseed")
        print_status "Reseeding database with fresh data..."
        $BINARY seed reseed
        ;;
    "help"|"-h"|"--help")
        echo "Usage: $0 [command]"
        echo ""
        echo "Commands:"
        echo "  seed    - Seed the database with mock user data (default)"
        echo "  clear   - Clear all users from the database"
        echo "  count   - Show the current number of users in the database"
        echo "  reseed  - Clear existing data and insert fresh mock data"
        echo "  help    - Show this help message"
        echo ""
        echo "Examples:"
        echo "  $0              # Seed the database"
        echo "  $0 seed         # Seed the database"
        echo "  $0 clear        # Clear all users"
        echo "  $0 count        # Count current users"
        echo "  $0 reseed       # Clear and reseed"
        ;;
    *)
        print_error "Unknown command: $1"
        echo "Run '$0 help' for available commands."
        exit 1
        ;;
esac

print_status "Operation completed successfully!"