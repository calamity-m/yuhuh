use async_trait::async_trait;
use sqlx::PgPool;
use tracing::error;

use crate::{error::YuhuhError, mood::model::MoodEntry};

// =============================================================================
// Traits
// =============================================================================

#[async_trait]
pub trait CreateMoodEntryRepository: std::fmt::Debug + Send + Sync + 'static {
    async fn create_mood_entry(&self, entry: MoodEntry) -> Result<(), YuhuhError>;
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
    async fn create_mood_entry(&self, entry: MoodEntry) -> Result<(), YuhuhError> {
        sqlx::query!(
            r#"
            INSERT INTO mood_records (
                user_id,
                created_at,
                mood,
                energy,
                sleep,
                notes
            )
            VALUES ($1, $2, $3, $4, $5, $6)
            "#,
            entry.user_id,
            entry.created_at.map(|dt| dt.to_utc()),
            entry.mood.map(|r| r.get() as i16),
            entry.energy.map(|r| r.get() as i16),
            entry.sleep.map(|r| r.get() as i16),
            entry.notes
        )
        .execute(&self.db)
        .await
        .map_err(|e| {
            error!(error = ?e, "database error while creating mood entry");

            YuhuhError::DatabaseError(e)
        })?;

        Ok(())
    }
}
