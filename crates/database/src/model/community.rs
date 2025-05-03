use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Community {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
    #[serde(rename = "creatorId")]
    pub creator_id: String,
    #[serde(rename = "lastMessageTime")]
    pub last_message_time: Option<DateTime<Utc>>,
    #[serde(rename = "contractAddress")]
    pub contract_address: Option<String>,
    #[serde(rename = "bountyAmount")]
    pub bounty_amount: Decimal,
    #[serde(rename = "timeLimit")]
    pub time_limit: Option<i32>,
    #[serde(rename = "baseFeePercentage")]
    pub base_fee_percentage: Option<f32>,
    #[serde(rename = "walletAddress")]
    pub wallet_address: Option<String>,
    #[serde(rename = "imageURL")]
    pub image_url: Option<String>,
}
