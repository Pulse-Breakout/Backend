use pulse_database::model::user::{User, CreateUserDto, UpdateUserDto};
use pulse_database::repository::user_repository::UserRepository;
use sqlx::{Pool, Postgres};
use thiserror::Error;
use uuid::Uuid;

#[derive(Error, Debug)]
pub enum UserServiceError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("User not found")]
    NotFound,

    #[error("Email already exists")]
    EmailExists,
}

pub struct UserService;

impl UserService {
    pub async fn get_all_users(pool: &Pool<Postgres>) -> Result<Vec<User>, UserServiceError> {
        let users = UserRepository::find_all(pool).await?;
        Ok(users)
    }

    pub async fn get_user_by_id(pool: &Pool<Postgres>, id: Uuid) -> Result<User, UserServiceError> {
        let user = UserRepository::find_by_id(pool, id).await?;
        user.ok_or(UserServiceError::NotFound)
    }

    pub async fn get_user_by_email(pool: &Pool<Postgres>, email: &str) -> Result<User, UserServiceError> {
        let user = UserRepository::find_by_email(pool, email).await?;
        user.ok_or(UserServiceError::NotFound)
    }

    pub async fn create_user(pool: &Pool<Postgres>, dto: CreateUserDto) -> Result<User, UserServiceError> {
        // Check if email already exists
        if let Ok(Some(_)) = UserRepository::find_by_email(pool, &dto.email).await {
            return Err(UserServiceError::EmailExists);
        }

        let user = UserRepository::create(pool, dto).await?;
        Ok(user)
    }


    pub async fn delete_user(pool: &Pool<Postgres>, id: Uuid) -> Result<(), UserServiceError> {
        let deleted = UserRepository::delete(pool, id).await?;
        if deleted {
            Ok(())
        } else {
            Err(UserServiceError::NotFound)
        }
    }
}