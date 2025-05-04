use sqlx::{Pool, Postgres};
use uuid::Uuid;
use chrono::Utc;

use crate::model::content::{Content, CreateContentDto};

pub struct ContentRepository;

impl ContentRepository {
    /// Find all content
    pub async fn find_all(pool: &Pool<Postgres>) -> Result<Vec<Content>, sqlx::Error> {
        let contents = sqlx::query_as!(
            Content,
            r#"
            SELECT 
                id, content, created_at, sender_id, sender_xid, image_url, community_id, wallet_address
            FROM content
            ORDER BY created_at DESC
            "#
        )
            .fetch_all(pool)
            .await?;

        Ok(contents)
    }

    /// Find content by ID
    pub async fn find_by_id(pool: &Pool<Postgres>, id: Uuid) -> Result<Option<Content>, sqlx::Error> {
        let content = sqlx::query_as!(
            Content,
            r#"
            SELECT 
                id, content, created_at, sender_id, sender_xid, image_url, community_id, wallet_address
            FROM content WHERE id = $1
            "#,
            id
        )
            .fetch_optional(pool)
            .await?;

        Ok(content)
    }

    /// Find content by community ID
    pub async fn find_by_community_id(pool: &Pool<Postgres>, community_id: Uuid) -> Result<Vec<Content>, sqlx::Error> {
        let contents = sqlx::query_as!(
            Content,
            r#"
            SELECT 
                id, content, created_at, sender_id, sender_xid, image_url, community_id, wallet_address
            FROM content WHERE community_id = $1
            ORDER BY created_at DESC
            "#,
            community_id
        )
            .fetch_all(pool)
            .await?;

        Ok(contents)
    }

    /// Create a new content
    pub async fn create(pool: &Pool<Postgres>, dto: CreateContentDto) -> Result<Content, sqlx::Error> {
        let now = Utc::now();
        let id = Uuid::new_v4(); // 새 UUID 생성

        // Default sender IDs if not provided
        let default_uuid = Uuid::parse_str("00000000-0000-0000-0000-000000000000").unwrap();
        let sender_id = dto.sender_id.unwrap_or(default_uuid);
        let sender_xid = dto.sender_xid.unwrap_or_else(|| "default-user".to_string());
        let wallet_address = dto.wallet_address.unwrap_or_else(|| "".to_string());

        let content = sqlx::query_as!(
        Content,
        r#"
        INSERT INTO content (
            id,           -- ID 필드 추가
            content, 
            sender_id, 
            sender_xid, 
            image_url, 
            community_id, 
            wallet_address,
            created_at
        )
        VALUES (
            $1, $2, $3, $4, $5, $6, $7, $8
        )
        RETURNING id, content, sender_id, sender_xid, image_url, community_id, wallet_address, created_at
        "#,
        id,              
        dto.content,
        sender_id,
        sender_xid,
        dto.image_url,
        dto.community_id,
        wallet_address,
        now
    )
            .fetch_one(pool)
            .await?;

        Ok(content)
    }

    /// Delete content
    pub async fn delete(pool: &Pool<Postgres>, id: Uuid) -> Result<bool, sqlx::Error> {
        let result = sqlx::query!(
            r#"
            DELETE FROM content WHERE id = $1
            "#,
            id
        )
            .execute(pool)
            .await?;

        Ok(result.rows_affected() > 0)
    }
}
