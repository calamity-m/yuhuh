use core::fmt;

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
    Ebiking,
    MountainBiking,
    Other,
}

impl fmt::Display for ActivityType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ActivityType::WeightLifting => write!(f, "WeightLifting"),
            ActivityType::Walking => write!(f, "Walking"),
            ActivityType::Jogging => write!(f, "Jogging"),
            ActivityType::Running => write!(f, "Running"),
            ActivityType::Cycling => write!(f, "Cycling"),
            ActivityType::Ebiking => write!(f, "Ebiking"),
            ActivityType::MountainBiking => write!(f, "MountainBiking"),
            ActivityType::Other => write!(f, "Other"),
        }
    }
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
