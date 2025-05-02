use axum::{routing::get, Router};

pub fn add2(left: u64, right: u64) -> u64 {
    left + right
}

// Define a simple handler function
async fn hello_world() -> &'static str {
    "Hello, World!"
}

// Create and configure the application router
pub fn create_router() -> Router {
    Router::new()
        .route("/", get(hello_world))
        .route("/api/health", get(|| async { "OK" }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add2(2, 2);
        assert_eq!(result, 4);
    }
}
