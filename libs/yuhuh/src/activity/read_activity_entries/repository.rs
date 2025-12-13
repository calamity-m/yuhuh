use async_trait::async_trait;
use chrono::{DateTime, NaiveDateTime, Utc};
use sqlx::PgPool;
use tracing::{debug, error, info};
use uuid::Uuid;

use crate::{activity::model::ActivityEntry, error::YuhuhError};

// =============================================================================
// Traits
// =============================================================================

#[async_trait]
pub trait ReadActivityEntriesRepository: std::fmt::Debug + Send + Sync + 'static {
    async fn read_activity_entries(
        &self,
        user_id: &Uuid,
        before: Option<DateTime<Utc>>,
        after: Option<DateTime<Utc>>,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<ActivityEntry>, YuhuhError>;
}

// =============================================================================
// Production Implementation
// =============================================================================

#[derive(Debug)]
pub struct ReadActivityEntriesRepositoryImpl {
    pub db: PgPool,
}

impl ReadActivityEntriesRepositoryImpl {
    pub fn new(db: PgPool) -> Self {
        ReadActivityEntriesRepositoryImpl { db }
    }
}

#[async_trait]
impl ReadActivityEntriesRepository for ReadActivityEntriesRepositoryImpl {
    async fn read_activity_entries(
        &self,
        user_id: &Uuid,
        before: Option<DateTime<Utc>>,
        after: Option<DateTime<Utc>>,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<ActivityEntry>, YuhuhError> {
        debug!(
            user_id=?user_id,
            before=?before,
            after=?after,
            limit=?limit,
            offset=?offset,
            "received read activity entries"
        );

        let activity_entries: Vec<ActivityEntry> = sqlx::query_as!(
            FoodEntry,
            r#"
            SELECT *
            FROM activity_records
            WHERE user_id = $1::uuid
            AND ($2::timestamptz IS NULL
                OR logged_at <= $2::timestamptz)
            AND ($3::timestamptz IS NULL
                OR logged_at >= $3::timestamptz)
            ORDER BY logged_at DESC
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
            error!(error = ?e, "database error while finding food records");

            YuhuhError::DatabaseError(e)
        })?;

        debug!(activity_entries=?activity_entries, "found activity records");

        Ok(activity_entries)
    }
}
