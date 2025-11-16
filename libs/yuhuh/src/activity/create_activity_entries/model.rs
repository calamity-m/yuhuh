use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub enum ActivityType {
    WeightLifting,
    Walking,
    Jogging,
    Running,
    Cycling,
    Ebike,
    MountainBiking,
    Other,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, sqlx::FromRow)]
pub struct ActivityEntry {
    pub activity_record_id: Option<Uuid>,
    pub user_id: Uuid,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
    pub activity: String,
    pub activity_type: ActivityType,
    pub activity_info: serde_json::Value,
}
