use async_trait::async_trait;
use chrono::NaiveDateTime;
use sqlx::PgPool;
use tracing::{error, info};
use uuid::Uuid;

use crate::{activity::model::ActivityEntry, error::YuhuhError, mood::model::MoodEntry};

// =============================================================================
// Traits
// =============================================================================

#[async_trait]
pub trait CreateActivityEntriesRepository: std::fmt::Debug + Send + Sync + 'static {
    async fn create_activity_entries(&self, entries: Vec<ActivityEntry>) -> Result<(), YuhuhError>;
}

// =============================================================================
// Production Implementation
// =============================================================================

#[derive(Debug)]
pub struct CreateActivityEntriesRepositoryImpl {
    pub db: PgPool,
}

impl CreateActivityEntriesRepositoryImpl {
    pub fn new(db: PgPool) -> Self {
        CreateActivityEntriesRepositoryImpl { db }
    }
}

#[async_trait]
impl CreateActivityEntriesRepository for CreateActivityEntriesRepositoryImpl {
    async fn create_activity_entries(&self, entries: Vec<ActivityEntry>) -> Result<(), YuhuhError> {
        Err(YuhuhError::NotImplemented)
    }
}
