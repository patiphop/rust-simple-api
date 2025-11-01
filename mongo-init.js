// MongoDB initialization script for Docker container
// This script runs when the container first starts

print("Starting MongoDB initialization for simple_api_db...");

try {
  // Switch to the application database
  db = db.getSiblingDB('simple_api_db');
  print("Switched to database: simple_api_db");

  // Check if user already exists before creating
  try {
    var existingUsers = db.getUsers ? db.getUsers() : [];
    var userExists = existingUsers.some(function(user) {
      return user.user === "api_user";
    });
  } catch (e) {
    print("Warning: Could not check existing users, proceeding with user creation: " + e.message);
    var userExists = false;
  }

  if (userExists) {
    print("User 'api_user' already exists, skipping user creation");
  } else {
    // Create application user with read/write permissions
    print("Creating user 'api_user' with readWrite permissions...");
    db.createUser({
      user: "admin",
      pwd: "admin",
      roles: [
        {
          role: "readWrite",
          db: "simple_api_db"
        }
      ]
    });
    print("✅ User 'api_user' created successfully");
  }

  // Create initial collections with error handling
  try {
    db.createCollection("users");
    print("✅ Collection 'users' created successfully");
  } catch (collectionError) {
    if (collectionError.code === 48) { // Collection already exists
      print("Collection 'users' already exists, skipping creation");
    } else {
      print("⚠️ Warning: Failed to create collection 'users': " + collectionError.message);
    }
  }

  // Verify user can authenticate and perform basic operations
  print("Verifying user permissions...");
  try {
    // Test authentication
    db.auth("api_user", "api_password");
    print("✅ User authentication successful");
    
    // Test basic operations
    var testCollection = "test_verification";
    db.createCollection(testCollection);
    db[testCollection].insertOne({test: "verification", timestamp: new Date()});
    var testDoc = db[testCollection].findOne();
    if (testDoc && testDoc.test === "verification") {
      print("✅ Database operations verification successful");
    }
    db[testCollection].drop();
    print("✅ Verification cleanup completed");
    
  } catch (verificationError) {
    print("❌ User verification failed: " + verificationError.message);
    throw verificationError;
  }

  print("🎉 MongoDB initialization completed successfully for simple_api_db");
  
} catch (error) {
  print("❌ MongoDB initialization failed: " + error.message);
  print("Error details: " + JSON.stringify(error));
  throw error;
}