use model;
use handlers;
use routes;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    // Get the router from routes crate
    let app = routes::create_router();

    // Run it with hyper on localhost:8080
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    
    

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
