use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct Rating(u32);

impl Rating {
    pub fn new(value: u32) -> Option<Rating> {
        if value <= 5 { Some(Self(value)) } else { None }
    }

    pub fn get(&self) -> u32 {
        self.0
    }
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct MoodAssignment {
    pub mood_assignment_id: Option<Uuid>,
    pub user_id: Uuid,
    pub val: String,
    pub idx: Rating,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct EnergyAssignment {
    pub energy_assignment_id: Option<Uuid>,
    pub user_id: Uuid,
    pub val: String,
    pub idx: Rating,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct SleepAssignment {
    pub sleep_assignment_id: Option<Uuid>,
    pub user_id: Uuid,
    pub val: String,
    pub idx: Rating,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct MoodEntry {
    pub mood_record_id: Option<Uuid>,
    pub user_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub mood: MoodAssignment,
    pub energy: EnergyAssignment,
    pub sleep: SleepAssignment,
    pub notes: Option<String>,
}

pub struct MoodTrend {}
