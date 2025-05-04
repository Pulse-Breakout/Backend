use axum::{
    extract::{State},
    http::StatusCode,
    Json,
};
use pulse_database::model::community::{Community, CreateCommunityDto};
use pulse_service::CommunityService;
use std::sync::Arc;
use pulse_database::connection::Database;

// Error handling for community handlers
pub enum CommunityHandlerError {
    Service(String),
}

// Convert CommunityHandlerError to StatusCode and message
impl axum::response::IntoResponse for CommunityHandlerError {
    fn into_response(self) -> axum::response::Response {
        let (status, message) = match self {
            CommunityHandlerError::Service(err) => {
                (StatusCode::INTERNAL_SERVER_ERROR, err)
            },
        };

        let body = Json(serde_json::json!({
            "error": message
        }));

        (status, body).into_response()
    }
}

// Convert service errors to CommunityHandlerError
impl From<String> for CommunityHandlerError {
    fn from(err: String) -> Self {
        CommunityHandlerError::Service(err)
    }
}

// Create new community
pub async fn create_community(
    State(db): State<Arc<Database>>,
    Json(dto): Json<CreateCommunityDto>,
) -> Result<Json<Community>, CommunityHandlerError> {
    // In a real application, you would extract the user_id from the authentication token
    // For now, we'll use the first user ID from our database
    let user_id = "2c81af65-e93a-4c28-a1e3-8ea22df8dec9".to_string();
    
    let service = CommunityService::new(db);
    let community = service.create_community(user_id, dto).await?;
    
    Ok(Json(community))
}