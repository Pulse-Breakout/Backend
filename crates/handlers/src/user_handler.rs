use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use pulse_database::model::user::{CreateUserDto, UpdateUserDto, User};
use pulse_service::UserService;
// No need for these imports
use std::sync::Arc;
use uuid::Uuid;
use pulse_database::connection::Database;

// Error handling for user handlers
pub enum UserHandlerError {
    Service(pulse_service::user_service::UserServiceError),
    InvalidUuid,
}

// Convert UserHandlerError to StatusCode and message
impl axum::response::IntoResponse for UserHandlerError {
    fn into_response(self) -> axum::response::Response {
        let (status, message) = match self {
            UserHandlerError::Service(err) => match err {
                pulse_service::user_service::UserServiceError::NotFound => {
                    (StatusCode::NOT_FOUND, "User not found".to_string())
                }
                pulse_service::user_service::UserServiceError::EmailExists => {
                    (StatusCode::CONFLICT, "Email already exists".to_string())
                }
                pulse_service::user_service::UserServiceError::Database(e) => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Database error: {}", e),
                ),
            },
            UserHandlerError::InvalidUuid => (
                StatusCode::BAD_REQUEST,
                "Invalid UUID format".to_string(),
            ),
        };

        let body = Json(serde_json::json!({
            "error": message
        }));

        (status, body).into_response()
    }
}

// Convert errors from various sources to UserHandlerError
impl From<pulse_service::user_service::UserServiceError> for UserHandlerError {
    fn from(err: pulse_service::user_service::UserServiceError) -> Self {
        UserHandlerError::Service(err)
    }
}

impl From<uuid::Error> for UserHandlerError {
    fn from(_: uuid::Error) -> Self {
        UserHandlerError::InvalidUuid
    }
}

// Get all users
pub async fn get_users(
    State(db): State<Arc<Database>>,
) -> Result<Json<Vec<User>>, UserHandlerError> {
    let users = UserService::get_all_users(db.pool()).await?;
    Ok(Json(users))
}

// Get user by ID
pub async fn get_user(
    State(db): State<Arc<Database>>,
    Path(id): Path<String>,
) -> Result<Json<User>, UserHandlerError> {
    let uuid = Uuid::parse_str(&id)?;
    let user = UserService::get_user_by_id(db.pool(), uuid).await?;
    Ok(Json(user))
}

// Create new user
pub async fn create_user(
    State(db): State<Arc<Database>>,
    Json(dto): Json<CreateUserDto>,
) -> Result<Json<User>, UserHandlerError> {
    let user = UserService::create_user(db.pool(), dto).await?;
    Ok(Json(user))
}

// Update user
pub async fn update_user(
    State(db): State<Arc<Database>>,
    Path(id): Path<String>,
    Json(dto): Json<UpdateUserDto>,
) -> Result<Json<User>, UserHandlerError> {
    let uuid = Uuid::parse_str(&id)?;
    let user = UserService::update_user(db.pool(), uuid, dto).await?;
    Ok(Json(user))
}

// Delete user
pub async fn delete_user(
    State(db): State<Arc<Database>>,
    Path(id): Path<String>,
) -> Result<StatusCode, UserHandlerError> {
    let uuid = Uuid::parse_str(&id)?;
    UserService::delete_user(db.pool(), uuid).await?;
    Ok(StatusCode::NO_CONTENT)
}