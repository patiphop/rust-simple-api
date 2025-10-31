mod db;
mod models;
mod handlers;

use dotenv::dotenv;
use std::env;
use std::sync::Arc;
use warp::Filter;

/// Default server port
const DEFAULT_PORT: u16 = 3030;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables from .env file
    dotenv().ok();
    
    // Initialize logger
    env_logger::init();
    
    // Check for seed command
    if env::args().len() > 1 && env::args().nth(1).unwrap_or_default() == "seed" {
        return handle_seed_command(&env::args().collect::<Vec<_>>()).await;
    }
    
    println!("Rust Simple API started!");
    
    // Initialize database connection
    let database = Arc::new(db::connect_to_database().await?);
    println!("Database connection established successfully!");
    
    // Check if we should seed data on startup (via environment variable)
    if env::var("SEED_ON_STARTUP").unwrap_or_default() == "true" {
        println!("Seeding data on startup...");
        match db::seed_users(&database).await {
            Ok(count) => {
                if count > 0 {
                    println!("Seeded {} users on startup", count);
                }
            }
            Err(e) => eprintln!("Error seeding data on startup: {}", e),
        }
    }
    
    // Get server port from environment variable or use default
    let port: u16 = env::var("PORT")
        .unwrap_or_else(|_| DEFAULT_PORT.to_string())
        .parse()
        .unwrap_or(DEFAULT_PORT);
    
    // Configure routes
    let health_route = warp::path("health")
        .and(warp::get())
        .and_then(handlers::health_check_with_status);
    
    // User routes with database access
    let db = database.clone();
    let users_get_all = warp::path("users")
        .and(warp::get())
        .and(warp::path::end())
        .and(warp::any().map(move || db.clone()))
        .and_then(handlers::get_all_users);
    
    let db = database.clone();
    let users_get_by_id = warp::path!("users" / String)
        .and(warp::get())
        .and(warp::any().map(move || db.clone()))
        .and_then(handlers::get_user_by_id);
    
    let db = database.clone();
    let users_create = warp::path("users")
        .and(warp::post())
        .and(warp::body::json())
        .and(warp::any().map(move || db.clone()))
        .and_then(handlers::create_user);
    
    let routes = health_route
        .or(users_get_all)
        .or(users_get_by_id)
        .or(users_create)
        .with(warp::cors().allow_any_origin());
    
    println!("Starting server on port {}", port);
    
    // Start the web server
    warp::serve(routes)
        .run(([0, 0, 0, 0], port))
        .await;
    
    Ok(())
}

/// Handle seed-related CLI commands
async fn handle_seed_command(args: &[String]) -> Result<(), Box<dyn std::error::Error>> {
    // Initialize database connection
    let database = db::connect_to_database().await?;
    
    match args.get(2).map(|s| s.as_str()) {
        Some("clear") => {
            println!("Clearing all users from database...");
            let deleted = db::clear_users(&database).await?;
            println!("Deleted {} users", deleted);
        }
        Some("count") => {
            let count = db::get_user_count(&database).await?;
            println!("Current user count: {}", count);
        }
        Some("reseed") => {
            println!("Reseeding database with fresh data...");
            let count = db::reseed_users(&database).await?;
            println!("Reseeded {} users", count);
        }
        None | Some("seed") => {
            println!("Seeding database with mock user data...");
            let count = db::seed_users(&database).await?;
            println!("Seeded {} users", count);
        }
        Some(cmd) => {
            eprintln!("Unknown seed command: {}", cmd);
            eprintln!("Available commands: seed, clear, count, reseed");
            return Err("Invalid seed command".into());
        }
    }
    
    Ok(())
}
