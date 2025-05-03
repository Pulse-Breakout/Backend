use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Depositor {
    pub id: Uuid,
    #[serde(rename = "userId")]
    pub user_id: String,
    #[serde(rename = "communityId")]
    pub community_id: String,
    pub amount: Decimal,
    #[serde(rename = "walletAddress")]
    pub wallet_address: Option<String>,
    #[serde(rename = "depositedAt")]
    pub deposited_at: DateTime<Utc>,
}
