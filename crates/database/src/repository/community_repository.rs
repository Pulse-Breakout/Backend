use sqlx::{Pool, Postgres};
use uuid::Uuid;
use chrono::Utc;
use rust_decimal::Decimal;

use crate::model::community::{Community, CreateCommunityDto, UpdateCommunityDto};

pub struct CommunityRepository;

impl CommunityRepository {
    /// Find all communities
    pub async fn find_all(pool: &Pool<Postgres>) -> Result<Vec<Community>, sqlx::Error> {
        let communities = sqlx::query_as!(
            Community,
            r#"
            SELECT 
                id, name, description, created_at, creator_id, last_message_time, 
                contract_address, bounty_amount, time_limit, 
                base_fee_percentage, wallet_address, image_url
            FROM communities
            "#
        )
            .fetch_all(pool)
            .await?;

        Ok(communities)
    }

    /// Find a community by ID
    pub async fn find_by_id(pool: &Pool<Postgres>, id: Uuid) -> Result<Option<Community>, sqlx::Error> {
        let community = sqlx::query_as!(
            Community,
            r#"
            SELECT 
                id, name, description, created_at, creator_id, last_message_time, 
                contract_address, bounty_amount, time_limit, 
                base_fee_percentage, wallet_address, image_url
            FROM communities WHERE id = $1
            "#,
            id
        )
            .fetch_optional(pool)
            .await?;

        Ok(community)
    }

    /// Find communities by creator ID
    pub async fn find_by_creator_id(pool: &Pool<Postgres>, creator_id: &str) -> Result<Vec<Community>, sqlx::Error> {
        let communities = sqlx::query_as!(
            Community,
            r#"
            SELECT 
                id, name, description, created_at, creator_id, last_message_time, 
                contract_address, bounty_amount, time_limit, 
                base_fee_percentage, wallet_address, image_url
            FROM communities WHERE creator_id = $1
            "#,
            creator_id
        )
            .fetch_all(pool)
            .await?;

        Ok(communities)
    }

    /// Create a new community
    pub async fn create(pool: &Pool<Postgres>, dto: CreateCommunityDto) -> Result<Community, sqlx::Error> {
        let id = Uuid::new_v4();
        let now = Utc::now();
        let bounty_amount = dto.bounty_amount.unwrap_or(Decimal::new(0, 0));

        let community = sqlx::query_as!(
            Community,
            r#"
            INSERT INTO communities (
                id, name, description, created_at, creator_id,
                contract_address, bounty_amount, time_limit,
                base_fee_percentage, wallet_address, image_url
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
            RETURNING 
                id, name, description, created_at, creator_id, last_message_time,
                contract_address, bounty_amount, time_limit,
                base_fee_percentage, wallet_address, image_url
            "#,
            id,
            dto.name,
            dto.description,
            now,
            dto.creator_id.unwrap_or_else(|| "default-user".to_string()),
            dto.contract_address,
            bounty_amount,
            dto.time_limit,
            dto.base_fee_percentage,
            dto.wallet_address,
            dto.image_url
        )
            .fetch_one(pool)
            .await?;

        Ok(community)
    }

    /// Update a community
    pub async fn update(pool: &Pool<Postgres>, id: Uuid, dto: UpdateCommunityDto) -> Result<Option<Community>, sqlx::Error> {
        // First, check if the community exists
        let existing = Self::find_by_id(pool, id).await?;
        if existing.is_none() {
            return Ok(None);
        }

        let community = sqlx::query_as!(
            Community,
            r#"
            UPDATE communities
            SET 
                name = COALESCE($1, name),
                description = COALESCE($2, description),
                last_message_time = COALESCE($3, last_message_time),
                contract_address = COALESCE($4, contract_address),
                bounty_amount = COALESCE($5, bounty_amount),
                time_limit = COALESCE($6, time_limit),
                base_fee_percentage = COALESCE($7, base_fee_percentage),
                wallet_address = COALESCE($8, wallet_address),
                image_url = COALESCE($9, image_url)
            WHERE id = $10
            RETURNING 
                id, name, description, created_at, creator_id, last_message_time,
                contract_address, bounty_amount, time_limit,
                base_fee_percentage, wallet_address, image_url
            "#,
            dto.name,
            dto.description,
            dto.last_message_time,
            dto.contract_address,
            dto.bounty_amount,
            dto.time_limit,
            dto.base_fee_percentage,
            dto.wallet_address,
            dto.image_url,
            id
        )
            .fetch_optional(pool)
            .await?;

        Ok(community)
    }

    /// Delete a community
    pub async fn delete(pool: &Pool<Postgres>, id: Uuid) -> Result<bool, sqlx::Error> {
        let result = sqlx::query!(
            r#"
            DELETE FROM communities WHERE id = $1
            "#,
            id
        )
            .execute(pool)
            .await?;

        Ok(result.rows_affected() > 0)
    }

    /// Update last message time
    pub async fn update_last_message_time(pool: &Pool<Postgres>, id: Uuid) -> Result<bool, sqlx::Error> {
        let now = Utc::now();
        let result = sqlx::query!(
            r#"
            UPDATE communities
            SET last_message_time = $1
            WHERE id = $2
            "#,
            now,
            id
        )
            .execute(pool)
            .await?;

        Ok(result.rows_affected() > 0)
    }
}