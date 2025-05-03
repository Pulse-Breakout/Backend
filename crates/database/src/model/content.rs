use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Content {
    pub id: Uuid,
    pub content: String,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
    #[serde(rename = "senderId")]
    pub sender_id: String,
    #[serde(rename = "imageURL")]
    pub image_url: Option<String>,
    #[serde(rename = "communityId")]
    pub community_id: String,
}
