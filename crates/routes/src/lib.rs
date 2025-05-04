use axum::{
    routing::{get, post, delete},
    Router,
    extract::State,
};
use std::sync::Arc;
use pulse_database::connection::Database;
use pulse_handlers::{user_handler, community_handler};

// Define a simple handler function
async fn hello_world() -> &'static str {
    "Hello, World!"
}

// Health check handler with database check
async fn health_check(State(db): State<Arc<Database>>) -> &'static str {
    match db.test_connection().await {
        Ok(_) => "OK",
        Err(_) => "Database connection failed",
    }
}

// Create and configure the application router
pub fn create_router(db: Arc<Database>) -> Router {
    Router::new()
        .route("/", get(hello_world))
        .route("/api/health", get(health_check))
        // User routes
        .route("/api/users", get(user_handler::get_users))
        .route("/api/users", post(user_handler::create_user))
        .route("/api/users/{id}", get(user_handler::get_user))
        .route("/api/users/{id}", delete(user_handler::delete_user))
        // Community routes
        .route("/api/communities", post(community_handler::create_community))
        .with_state(db)
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use tower::ServiceExt;
    use std::sync::Arc;


    #[tokio::test]
    async fn test_hello_world() {
        // Test the async function directly
        let result = hello_world().await;
        assert_eq!(result, "Hello, World!");
    }

    #[tokio::test]
    async fn test_router() {
        // Skip this test if we don't have a real database connection
        // In a real project, you'd use a test database or mock
        // For example purposes, we'll create a mock router without db requirement
        
        // Create a router without state for testing
        let app = Router::new()
            .route("/", get(hello_world))
            .route("/api/health", get(|| async { "OK" }));

        // Test root path
        let response = app
            .clone()
            .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);

        // Test health check path
        let response = app
            .oneshot(Request::builder().uri("/api/health").body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }
}
