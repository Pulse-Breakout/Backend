pub mod logbot;

use pulse_handlers;
use pulse_routes;
use pulse_database;
use std::net::SocketAddr;



#[tokio::main]
async fn main() {
    // Initialize the database connection


    let db = match pulse_database::connection::create_db_pool().await {
        Ok(pool) => {
            println!("✅ Connected to database successfully");
            pool
        },
        Err(e) => {
            eprintln!("❌ Failed to connect to database: {}", e);
            std::process::exit(1);
        }
    };
    
    // Get the router from routes crate with database connection
    let app = pulse_routes::create_router(db);

    // Run it with hyper on localhost:8080
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("🚀 Server starting on {}", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();




}
