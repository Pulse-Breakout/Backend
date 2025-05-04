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
                id, content, created_at, sender_id, image_url, community_id
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
                id, content, created_at, sender_id, image_url, community_id
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
                id, content, created_at, sender_id, image_url, community_id
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
        let id = Uuid::new_v4();
        let now = Utc::now();

        let content = sqlx::query_as!(
            Content,
            r#"
            INSERT INTO content (
                id, content, created_at, sender_id, image_url, community_id
            )
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING 
                id, content, created_at, sender_id, image_url, community_id
            "#,
            id,
            dto.content,
            now,
            dto.sender_id.unwrap_or_else(|| "default-user".to_string()),
            dto.image_url,
            dto.community_id
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