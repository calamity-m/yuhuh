use async_trait::async_trait;
use chrono::{DateTime, NaiveDateTime, Utc};
use sqlx::PgPool;
use tracing::{error, info};
use uuid::Uuid;

use crate::{
    error::YuhuhError,
    mood::model::{MoodEntry, MoodTrend},
};

// =============================================================================
// Traits
// =============================================================================

#[async_trait]
pub trait ReadMoodEntriesRepository: std::fmt::Debug + Send + Sync + 'static {
    async fn find_mood_entries(
        &self,
        user_id: &Uuid,
        before: Option<DateTime<Utc>>,
        after: Option<DateTime<Utc>>,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<MoodEntry>, YuhuhError>;

    async fn find_mood_trend(
        &self,
        user_id: &Uuid,
        from: Option<DateTime<Utc>>,
        to: Option<DateTime<Utc>>,
    ) -> Result<MoodTrend, YuhuhError>;
}

// =============================================================================
// Production Implementation
// =============================================================================

#[derive(Debug)]
pub struct ReadMoodEntriesRepositoryImpl {
    pub db: PgPool,
}

impl ReadMoodEntriesRepositoryImpl {
    pub fn new(db: PgPool) -> Self {
        ReadMoodEntriesRepositoryImpl { db }
    }
}

#[async_trait]
impl ReadMoodEntriesRepository for ReadMoodEntriesRepositoryImpl {
    async fn find_mood_entries(
        &self,
        user_id: &Uuid,
        before: Option<DateTime<Utc>>,
        after: Option<DateTime<Utc>>,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<MoodEntry>, YuhuhError> {
        Err(YuhuhError::NotImplemented)
    }

    async fn find_mood_trend(
        &self,
        user_id: &Uuid,
        from: Option<DateTime<Utc>>,
        to: Option<DateTime<Utc>>,
    ) -> Result<MoodTrend, YuhuhError> {
        Err(YuhuhError::NotImplemented)
    }
}
