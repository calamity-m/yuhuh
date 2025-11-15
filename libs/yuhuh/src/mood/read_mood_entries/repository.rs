use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::PgPool;
use tracing::{debug, error};
use uuid::Uuid;

use crate::{
    error::YuhuhError,
    mood::model::{MoodEntry, MoodEntryRow},
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
        debug!(
            user_id=?user_id,
            before=?before,
            after=?after,
            limit=?limit,
            offset=?offset,
            "received find request for mood entries"
        );

        let records: Vec<MoodEntryRow> = sqlx::query_as!(
            MoodEntryRow,
            r#"
            SELECT *
            FROM mood_records
            WHERE user_id = $1::uuid
            AND ($2::timestamptz IS NULL
                OR created_at <= $2::timestamptz)
            AND ($3::timestamptz IS NULL
                OR created_at >= $3::timestamptz)
            ORDER BY created_at DESC
            LIMIT $4
            OFFSET $5;
            "#,
            user_id,
            before,
            after,
            limit,
            offset
        )
        .fetch_all(&self.db)
        .await
        .map_err(|e| {
            error!(error = ?e, "database error while finding mood records");

            YuhuhError::DatabaseError(e)
        })?;

        debug!(mood_records=?records, "found mood records");

        let mut errors_found: bool = false;

        let mood_entries: Vec<MoodEntry> = records
            .into_iter()
            .filter_map(|row| {
                row.try_into()
                    .inspect_err(|e| {
                        error!(error=?e, "ecountered parsing error for mood entry");
                        errors_found = true;
                    })
                    .ok()
            })
            .collect();

        if errors_found {
            return Err(YuhuhError::InternalServerError(
                "internal server error occured reading mood entries".to_string(),
            ));
        }

        Ok(mood_entries)
    }
}
