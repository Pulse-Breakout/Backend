use sqlx::{Pool, Postgres};
use uuid::Uuid;
use chrono::Utc;


use crate::model::user::{User, CreateUserDto, UpdateUserDto};


pub struct UserRepository;

impl UserRepository {
    
    
    
    pub async fn find_all(pool: &Pool<Postgres>) -> Result<Vec<User>, sqlx::Error> {
        let users = sqlx::query_as!(
            User,
            r#"
            SELECT * FROM users
            "#
        )
            .fetch_all(pool)
            .await?;

        Ok(users)
    }

    pub async fn find_by_id(pool: &Pool<Postgres>, id: Uuid) -> Result<Option<User>, sqlx::Error> {
        let user = sqlx::query_as!(
            User,
            r#"
            SELECT * FROM users WHERE id = $1
            "#,
            id
        )
            .fetch_optional(pool)
            .await?;

        Ok(user)
    }

    pub async fn find_by_email(pool: &Pool<Postgres>, email: &str) -> Result<Option<User>, sqlx::Error> {
        let user = sqlx::query_as!(
            User,
            r#"
            SELECT * FROM users WHERE email = $1
            "#,
            email
        )
            .fetch_optional(pool)
            .await?;

        Ok(user)
    }

    pub async fn create(pool: &Pool<Postgres>, dto: CreateUserDto) -> Result<User, sqlx::Error> {
        // 실제 구현에서는 비밀번호 해싱 로직이 필요합니다
        let password_hash = dto.password; // 실제로는 해싱 필요

        let user = sqlx::query_as!(
            User,
            r#"
            INSERT INTO users (id, username, email, password_hash, created_at, updated_at, is_active)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING *
            "#,
            Uuid::new_v4(),
            dto.username,
            dto.email,
            password_hash,
            Utc::now(),
            Utc::now(),
            true
        )
            .fetch_one(pool)
            .await?;

        Ok(user)
    }

    pub async fn update(pool: &Pool<Postgres>, id: Uuid, dto: UpdateUserDto) -> Result<Option<User>, sqlx::Error> {
        // 현재 사용자 정보 조회
        let current_user = Self::find_by_id(pool, id).await?;

        if current_user.is_none() {
            return Ok(None);
        }

        let current_user = current_user.unwrap();

        // 업데이트할 필드 설정
        let username = dto.username.unwrap_or(current_user.username);
        let email = dto.email.unwrap_or(current_user.email);
        let password_hash = match dto.password {
            Some(password) => password, // 실제로는 해싱 필요
            None => current_user.password_hash,
        };
        let is_active = dto.is_active.unwrap_or(current_user.is_active);

        let updated_user = sqlx::query_as!(
            User,
            r#"
            UPDATE users
            SET username = $1, email = $2, password_hash = $3, updated_at = $4, is_active = $5
            WHERE id = $6
            RETURNING *
            "#,
            username,
            email,
            password_hash,
            Utc::now(),
            is_active,
            id
        )
            .fetch_optional(pool)
            .await?;

        Ok(updated_user)
    }

    pub async fn delete(pool: &Pool<Postgres>, id: Uuid) -> Result<bool, sqlx::Error> {
        let result = sqlx::query!(
            r#"
            DELETE FROM users WHERE id = $1
            "#,
            id
        )
            .execute(pool)
            .await?;

        Ok(result.rows_affected() > 0)
    }
}
