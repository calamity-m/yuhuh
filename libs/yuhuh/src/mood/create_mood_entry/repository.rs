use async_trait::async_trait;
use sqlx::PgPool;

use crate::{error::YuhuhError, mood::model::MoodEntry};

// =============================================================================
// Traits
// =============================================================================

#[async_trait]
pub trait CreateMoodEntryRepository: std::fmt::Debug + Send + Sync + 'static {
    async fn create_mood_entry(&self, entries: MoodEntry) -> Result<(), YuhuhError>;
}

// =============================================================================
// Production Implementation
// =============================================================================

#[derive(Debug)]
pub struct CreateMoodEntryRepositoryImpl {
    pub db: PgPool,
}

impl CreateMoodEntryRepositoryImpl {
    pub fn new(db: PgPool) -> Self {
        CreateMoodEntryRepositoryImpl { db }
    }
}

#[async_trait]
impl CreateMoodEntryRepository for CreateMoodEntryRepositoryImpl {
    async fn create_mood_entry(&self, entries: MoodEntry) -> Result<(), YuhuhError> {
        Err(YuhuhError::NotImplemented)
    }
}
