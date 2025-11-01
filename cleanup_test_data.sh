#!/bin/bash

# Cleanup script for test data
# This script removes test users from the database

echo "Cleaning up test data from database..."

# Connect to MongoDB and remove test users
docker exec -it rust-simple-api-mongodb mongosh --eval "
use simple_api_db;

// Remove users with test-specific patterns
db.users.deleteMany({
  '\$or': [
    { 'email': { '\$regex': '.*@test\\\\.com\$', '\$options': 'i' } },
    { 'name': { '\$regex': '.*Test.*', '\$options': 'i' } },
    { 'name': { '\$regex': '.*Integration.*', '\$options': 'i' } },
    { 'name': { '\$regex': '.*Concurrent.*', '\$options': 'i' } },
    { 'name': { '\$regex': '.*Database.*', '\$options': 'i' } }
  ]
});

// Count remaining users
var count = db.users.countDocuments({});
print('Remaining users after cleanup: ' + count);
"

echo "Test data cleanup completed!"