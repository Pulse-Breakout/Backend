use axum::{
    extract::{State, Path},
    http::StatusCode,
    Json,
};
use pulse_database::model::content::{Content, CreateContentDto};
use pulse_service::ContentService;
use std::sync::Arc;
use pulse_database::connection::Database;

// Error handling for content handlers
pub enum ContentHandlerError {
    Service(String),
    NotFound,
}

// Convert ContentHandlerError to StatusCode and message
impl axum::response::IntoResponse for ContentHandlerError {
    fn into_response(self) -> axum::response::Response {
        let (status, message) = match self {
            ContentHandlerError::Service(err) => {
                (StatusCode::INTERNAL_SERVER_ERROR, err)
            },
            ContentHandlerError::NotFound => {
                (StatusCode::NOT_FOUND, "Content not found".to_string())
            }
        };

        let body = Json(serde_json::json!({
            "error": message
        }));

        (status, body).into_response()
    }
}

// Convert service errors to ContentHandlerError
impl From<String> for ContentHandlerError {
    fn from(err: String) -> Self {
        ContentHandlerError::Service(err)
    }
}

// Create new content
pub async fn create_content(
    State(db): State<Arc<Database>>,
    Json(dto): Json<CreateContentDto>,
) -> Result<Json<Content>, ContentHandlerError> {
    // In a real application, you would extract the user_id from the authentication token
    // For now, we'll use the same test user ID
    let user_id = "2c81af65-e93a-4c28-a1e3-8ea22df8dec9".to_string();
    
    let service = ContentService::new(db);
    let content = service.create_content(user_id, dto).await?;
    
    Ok(Json(content))
}

// Get content by ID
pub async fn get_content(
    State(db): State<Arc<Database>>,
    Path(id): Path<String>,
) -> Result<Json<Content>, ContentHandlerError> {
    let service = ContentService::new(db);
    let content = service.get_content_by_id(id).await?
        .ok_or(ContentHandlerError::NotFound)?;
    
    Ok(Json(content))
}

// Get all content
pub async fn get_all_contents(
    State(db): State<Arc<Database>>,
) -> Result<Json<Vec<Content>>, ContentHandlerError> {
    let service = ContentService::new(db);
    let contents = service.get_all_contents().await?;
    
    Ok(Json(contents))
}

// Get content by community ID
pub async fn get_community_contents(
    State(db): State<Arc<Database>>,
    Path(community_id): Path<String>,
) -> Result<Json<Vec<Content>>, ContentHandlerError> {
    let service = ContentService::new(db);
    let contents = service.get_contents_by_community(community_id).await?;
    
    Ok(Json(contents))
}