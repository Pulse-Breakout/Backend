use std::sync::Arc;

use pulse_database::{
    connection::Database,
    model::content::{Content, CreateContentDto},
    repository::ContentRepository,
};
use uuid::Uuid;

pub struct ContentService {
    db: Arc<Database>,
}

impl ContentService {
    pub fn new(db: Arc<Database>) -> Self {
        Self { db }
    }

    pub async fn create_content(&self, user_id: String, mut dto: CreateContentDto) -> Result<Content, String> {
        // UUID 문자열을 UUID 타입으로 변환
        let uuid_id = match uuid::Uuid::parse_str(&user_id) {
            Ok(uuid) => uuid,
            Err(_) => return Err("Invalid UUID format".to_string()),
        };

        // 사용자의 xid 조회
        let user_xid = sqlx::query_scalar!(
            "SELECT xid FROM users WHERE id = $1",
            uuid_id
        )
            .fetch_one(self.db.pool())
            .await
            .map_err(|e| format!("Failed to get user xid: {}", e))?;

        // xid를 sender_id로 사용
        dto.sender_id = Some(user_xid);

        ContentRepository::create(self.db.pool(), dto)
            .await
            .map_err(|e| format!("Failed to create content: {}", e))
    }

    pub async fn get_content_by_id(&self, id: String) -> Result<Option<Content>, String> {
        let uuid_id = match uuid::Uuid::parse_str(&id) {
            Ok(uuid) => uuid,
            Err(_) => return Err("Invalid UUID format".to_string()),
        };

        ContentRepository::find_by_id(self.db.pool(), uuid_id)
            .await
            .map_err(|e| format!("Failed to get content: {}", e))
    }

    pub async fn get_all_contents(&self) -> Result<Vec<Content>, String> {
        ContentRepository::find_all(self.db.pool())
            .await
            .map_err(|e| format!("Failed to get contents: {}", e))
    }

    pub async fn get_contents_by_community(&self, community_id: String) -> Result<Vec<Content>, String> {
        let uuid_id = match uuid::Uuid::parse_str(&community_id) {
            Ok(uuid) => uuid,
            Err(_) => return Err("Invalid UUID format".to_string()),
        };

        ContentRepository::find_by_community_id(self.db.pool(), uuid_id)
            .await
            .map_err(|e| format!("Failed to get community contents: {}", e))
    }
}