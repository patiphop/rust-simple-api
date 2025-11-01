#!/bin/bash

# Docker MongoDB Setup Script for Rust Simple API
# This script manages MongoDB container using Docker Compose

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Function to print colored output
print_status() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Function to check if Docker is running
check_docker() {
    if ! docker info > /dev/null 2>&1; then
        print_error "Docker is not running. Please start Docker first."
        exit 1
    fi
}

# Function to stop native MongoDB service
stop_native_mongodb() {
    print_status "Checking for native MongoDB service..."
    
    # Check for MongoDB service on different platforms
    if command -v brew > /dev/null 2>&1; then
        # macOS with Homebrew
        if brew services list | grep mongodb-community | grep started > /dev/null 2>&1; then
            print_warning "Stopping native MongoDB service (Homebrew)..."
            brew services stop mongodb-community
            print_success "Native MongoDB service stopped."
        fi
    elif command -v systemctl > /dev/null 2>&1; then
        # Linux with systemd
        if systemctl is-active --quiet mongod 2>/dev/null; then
            print_warning "Stopping native MongoDB service (systemd)..."
            sudo systemctl stop mongod
            print_success "Native MongoDB service stopped."
        fi
    elif command -v sc > /dev/null 2>&1; then
        # Windows
        if sc query MongoDB 2>/dev/null | grep RUNNING > /dev/null; then
            print_warning "Stopping native MongoDB service (Windows)..."
            net stop MongoDB
            print_success "Native MongoDB service stopped."
        fi
    fi
    
    # Check if MongoDB process is still running on port 27017
    if lsof -i :27017 > /dev/null 2>&1; then
        print_warning "MongoDB is still running on port 27017. Attempting to kill process..."
        pkill -f mongod || true
        sleep 2
        if lsof -i :27017 > /dev/null 2>&1; then
            print_error "Could not stop MongoDB on port 27017. Please stop it manually."
            exit 1
        fi
    fi
}

# Function to start MongoDB container
start_mongodb() {
    print_status "Starting MongoDB container..."
    check_docker
    
    # Stop native MongoDB if running
    stop_native_mongodb
    
    # Start the container
    if docker-compose up -d; then
        print_success "MongoDB container started successfully."
        print_status "Waiting for MongoDB to be ready..."
        sleep 5
        
        # Check if container is running
        if docker-compose ps | grep -q "Up"; then
            print_success "MongoDB is ready and running on port 27017."
            print_status "Connection string: mongodb://admin:admin@localhost:27017/simple_api_db"
        else
            print_error "Failed to start MongoDB container."
            docker-compose logs
            exit 1
        fi
    else
        print_error "Failed to start MongoDB container."
        exit 1
    fi
}

# Function to stop MongoDB container
stop_mongodb() {
    print_status "Stopping MongoDB container..."
    
    if docker-compose down; then
        print_success "MongoDB container stopped successfully."
    else
        print_warning "MongoDB container was not running or failed to stop."
    fi
}

# Function to restart MongoDB container
restart_mongodb() {
    print_status "Restarting MongoDB container..."
    stop_mongodb
    sleep 2
    start_mongodb
}

# Function to show MongoDB container status
show_status() {
    print_status "MongoDB Container Status:"
    echo ""
    
    if docker-compose ps | grep -q "Up"; then
        print_success "MongoDB container is running."
        echo ""
        print_status "Container Details:"
        docker-compose ps
        echo ""
        print_status "Connection Information:"
        echo "  Host: localhost"
        echo "  Port: 27017"
        echo "  Database: simple_api_db"
        echo "  Username: admin"
        echo "  Password: admin"
        echo "  Connection String: mongodb://admin:admin@localhost:27017/simple_api_db"
    else
        print_warning "MongoDB container is not running."
        echo ""
        print_status "Container Status:"
        docker-compose ps || echo "No containers found."
    fi
}

# Function to show logs
show_logs() {
    print_status "MongoDB Container Logs:"
    docker-compose logs -f mongodb
}

# Function to show usage
show_usage() {
    echo "Usage: $0 {start|stop|restart|status|logs|help}"
    echo ""
    echo "Commands:"
    echo "  start   - Start MongoDB container (stops native MongoDB if running)"
    echo "  stop    - Stop MongoDB container"
    echo "  restart - Restart MongoDB container"
    echo "  status  - Show MongoDB container status and connection info"
    echo "  logs    - Show MongoDB container logs (follow mode)"
    echo "  help    - Show this help message"
    echo ""
    echo "Examples:"
    echo "  $0 start    # Start MongoDB with Docker"
    echo "  $0 status   # Check MongoDB status"
    echo "  $0 stop     # Stop MongoDB container"
}

# Main script logic
case "${1:-help}" in
    start)
        start_mongodb
        ;;
    stop)
        stop_mongodb
        ;;
    restart)
        restart_mongodb
        ;;
    status)
        show_status
        ;;
    logs)
        show_logs
        ;;
    help|--help|-h)
        show_usage
        ;;
    *)
        print_error "Invalid command: $1"
        echo ""
        show_usage
        exit 1
        ;;
esac