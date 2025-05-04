use sqlx::{PgPool, Pool, Postgres};
use uuid::Uuid;
use chrono::Utc;


use crate::model::user::{User, CreateUserDto, UpdateUserDto};


pub struct UserRepository;

impl UserRepository {
    
    
    
    pub async fn find_all(pool: &Pool<Postgres>) -> Result<Vec<User>, sqlx::Error> {
        let users = sqlx::query_as!(
            User,
            r#"
            SELECT 
                id, xid, username, profile_image_url, wallet_address, 
                email, created_at, updated_at
            FROM users
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
            SELECT 
                id, xid, username, profile_image_url, wallet_address, 
                email, created_at, updated_at
            FROM users WHERE id = $1
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
            SELECT 
                id, xid, username, profile_image_url, wallet_address, 
                email, created_at, updated_at
            FROM users WHERE email = $1
            "#,
            email
        )
            .fetch_optional(pool)
            .await?;

        Ok(user)
    }

    pub async fn create(pool: &Pool<Postgres>, dto: CreateUserDto) -> Result<User, sqlx::Error> {
        // In a real implementation, password hashing would be required
        // Password hashing would be done here
        let id = Uuid::new_v4();
        let xid = id.to_string();
        let now = Utc::now();

        let user = sqlx::query_as!(
            User,
            r#"
            INSERT INTO users (id, xid, username, profile_image_url, wallet_address, email, password_hash, created_at, updated_at, is_active)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            RETURNING id, xid, username, profile_image_url, wallet_address, email, created_at, updated_at
            "#,
            id,
            xid,
            dto.username,
            dto.profile_image_url.as_deref(),
            dto.wallet_address,
            dto.email,
            dto.password,
            now,
            now,
            true
        )
            .fetch_one(pool)
            .await?;

        Ok(user)
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



    pub async fn get_xid_by_id(pool: &PgPool, id: &str) -> Result<String, sqlx::Error> {
        // 문자열을 UUID로 파싱
        let uuid_id = match Uuid::parse_str(id) {
            Ok(uuid) => uuid,
            Err(_) => return Err(sqlx::Error::Protocol("Invalid UUID format".into())),
        };

        let xid = sqlx::query_scalar!("SELECT xid FROM users WHERE id = $1", uuid_id)
            .fetch_one(pool)
            .await?;

        Ok(xid)
    }

}
