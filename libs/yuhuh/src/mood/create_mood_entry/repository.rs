use async_trait::async_trait;
use chrono::NaiveDateTime;
use sqlx::PgPool;
use tracing::{error, info};
use uuid::Uuid;

use crate::{error::YuhuhError, mood::model::MoodEntry};

// =============================================================================
// Traits
// =============================================================================

#[async_trait]
pub trait EnterMoodEntryRepository: std::fmt::Debug + Send + Sync + 'static {
    async fn create_mood_entries(&self, entries: Vec<MoodEntry>) -> Result<(), YuhuhError>;
}

// =============================================================================
// Production Implementation
// =============================================================================

#[derive(Debug)]
pub struct EnterMoodEntryRepositoryImpl {
    pub db: PgPool,
}

impl EnterMoodEntryRepositoryImpl {
    pub fn new(db: PgPool) -> Self {
        EnterMoodEntryRepositoryImpl { db }
    }
}

#[async_trait]
impl EnterMoodEntryRepository for EnterMoodEntryRepositoryImpl {
    async fn create_mood_entries(&self, entries: Vec<MoodEntry>) -> Result<(), YuhuhError> {
        Err(YuhuhError::NotImplemented)
    }
}
