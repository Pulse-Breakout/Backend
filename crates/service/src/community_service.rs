use std::sync::Arc;

use pulse_database::{
    connection::Database,
    model::community::{Community, CreateCommunityDto},
    repository::CommunityRepository,
};

pub struct CommunityService {
    db: Arc<Database>,
}

impl CommunityService {
    pub fn new(db: Arc<Database>) -> Self {
        Self { db }
    }

    pub async fn create_community(&self, user_id: String, mut dto: CreateCommunityDto) -> Result<Community, String> {
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

        // uuid_id와 xid를 각각 설정
        dto.creator_id = Some(uuid_id);
        dto.creator_xid = Some(user_xid);

        CommunityRepository::create(self.db.pool(), dto)
            .await
            .map_err(|e| format!("Failed to create community: {}", e))
    }

    pub async fn get_all_communities(&self) -> Result<Vec<Community>, String> {
        CommunityRepository::find_all(self.db.pool())
            .await
            .map_err(|e| format!("Failed to get communities: {}", e))
    }
}
