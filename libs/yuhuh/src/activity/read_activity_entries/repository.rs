use async_trait::async_trait;
use chrono::{DateTime, NaiveDateTime, Utc};
use sqlx::PgPool;
use tracing::{error, info};
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
        Err(YuhuhError::NotImplemented)
    }
}
