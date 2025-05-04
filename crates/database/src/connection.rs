
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::env;
use std::sync::Arc;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DbError {
    #[error("Database error: {0}")]
    Sqlx(#[from] sqlx::Error),

    #[error("Environment variable error: {0}")]
    Env(#[from] std::env::VarError),

    #[error("Database connection failed: {0}")]
    Connection(String),
}

/// Database connection pool that can be shared across the application
pub struct Database {
    pool: Pool<Postgres>,
}

impl Database {
    /// Initialize a new database connection pool from environment variables
    pub async fn new() -> Result<Self, DbError> {
        // Load environment variables from .env file
        dotenvy::dotenv().ok();

        // Get the database URL from environment variables
        let database_url = env::var("DATABASE_URL")?;

        // Create the connection pool
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&database_url)
            .await
            .map_err(|e| DbError::Connection(e.to_string()))?;

        // Run migrations from the database crate
        sqlx::migrate!("./migrations")
            .run(&pool)
            .await
            .map_err(|e| DbError::Connection(format!("Migration failed: {}", e)))?;

        Ok(Self { pool })
    }

    /// Get a reference to the connection pool
    pub fn pool(&self) -> &Pool<Postgres> {
        &self.pool
    }

    /// Test the database connection
    pub async fn test_connection(&self) -> Result<(), DbError> {
        // A simple query to test if the connection works
        sqlx::query("SELECT 1")
            .execute(self.pool())
            .await?;

        Ok(())
    }
}

/// Create a shared database connection pool
pub async fn create_db_pool() -> Result<Arc<Database>, DbError> {
    let db = Database::new().await?;

    // Test the connection to ensure it works
    db.test_connection().await?;

    Ok(Arc::new(db))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_database_connection() {
        // This test only runs if the DATABASE_URL environment variable is set
        if let Ok(_) = env::var("DATABASE_URL") {
            let db = Database::new().await;
            assert!(db.is_ok(), "Database connection failed");

            if let Ok(db) = db {
                let test_result = db.test_connection().await;
                assert!(test_result.is_ok(), "Test query failed");
            }
        }
    }
}