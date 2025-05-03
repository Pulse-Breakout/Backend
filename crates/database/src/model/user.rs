use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: Uuid,
    pub xid: String, 
    pub username: String,
    #[serde(rename = "profileImageUrl")]
    pub profile_image_url: Option<String>,
    #[serde(rename = "walletAddress")]
    pub wallet_address: String,
    pub email: String,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
    #[serde(rename = "updatedAt")]
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUserDto {
    pub username: String,
    #[serde(rename = "profileImageUrl")]
    pub profile_image_url: Option<String>,
    #[serde(rename = "walletAddress")]
    pub wallet_address: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateUserDto {
    pub username: Option<String>,
    #[serde(rename = "profileImageUrl")]
    pub profile_image_url: Option<String>,
    #[serde(rename = "walletAddress")]
    pub wallet_address: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
}


