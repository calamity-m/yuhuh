use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct Rating(u32);

impl Rating {
    pub fn new(value: u32) -> Option<Rating> {
        if value <= 10 { Some(Self(value)) } else { None }
    }

    pub fn get(&self) -> u32 {
        self.0
    }
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct MoodEntry {
    pub mood_record_id: Option<Uuid>,
    pub user_id: Uuid,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
    pub mood: Option<Rating>,
    pub energy: Option<Rating>,
    pub sleep: Option<Rating>,
    pub notes: Option<String>,
}
