// MongoDB initialization script for Docker container
// This script runs when the container first starts

// Switch to the application database
db = db.getSiblingDB('simple_api_db');

// Create application user with read/write permissions
db.createUser({
  user: "api_user",
  pwd: "api_password",
  roles: [
    {
      role: "readWrite",
      db: "simple_api_db"
    }
  ]
});

// Create initial collections (optional)
db.createCollection("users");

// Note: Seed data is now handled by the Rust application in src/db/seed.rs
// This ensures a single source of truth for data seeding

print("MongoDB initialization completed for simple_api_db with seed data");