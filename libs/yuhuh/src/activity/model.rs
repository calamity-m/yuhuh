use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub enum ActivityType {
    WeightLifting,
    Walking,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct ActivityEntry {
    pub activity_record_id: Option<Uuid>,
    pub user_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub activity: String,
    pub activity_type: ActivityType,
    pub activity_info: serde_json::Value,
}
