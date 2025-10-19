use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Mood(u8);

impl Mood {
    pub fn new(value: u8) -> Option<Mood> {
        if value <= 10 { Some(Self(value)) } else { None }
    }

    pub fn get(&self) -> u8 {
        self.0
    }
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct MoodEntry {
    pub mood: Mood,
    pub mood_description: Option<String>,
    pub user_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub struct MoodTrend {}
