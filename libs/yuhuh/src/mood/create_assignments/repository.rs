use async_trait::async_trait;
use chrono::NaiveDateTime;
use sqlx::PgPool;
use tracing::{error, info};
use uuid::Uuid;

use crate::{
    error::YuhuhError,
    mood::model::{EnergyAssignment, MoodAssignment, MoodEntry, SleepAssignment},
};

// =============================================================================
// Traits
// =============================================================================

#[async_trait]
pub trait CreateAssignmentRepository: std::fmt::Debug + Send + Sync + 'static {
    async fn upsert_mood_assignment(&self, assignment: MoodAssignment) -> Result<(), YuhuhError>;

    async fn upsert_energy_assignment(
        &self,
        assignment: EnergyAssignment,
    ) -> Result<(), YuhuhError>;

    async fn upsert_sleep_assignment(&self, assignment: SleepAssignment) -> Result<(), YuhuhError>;
}

// =============================================================================
// Production Implementation
// =============================================================================

#[derive(Debug)]
pub struct CreateAssignmentRepositoryImpl {
    pub db: PgPool,
}

impl CreateAssignmentRepositoryImpl {
    pub fn new(db: PgPool) -> Self {
        CreateAssignmentRepositoryImpl { db }
    }
}

#[async_trait]
impl CreateAssignmentRepository for CreateAssignmentRepositoryImpl {
    async fn upsert_mood_assignment(&self, assignment: MoodAssignment) -> Result<(), YuhuhError> {
        Err(YuhuhError::NotImplemented)
    }

    async fn upsert_energy_assignment(
        &self,
        assignment: EnergyAssignment,
    ) -> Result<(), YuhuhError> {
        Err(YuhuhError::NotImplemented)
    }

    async fn upsert_sleep_assignment(&self, assignment: SleepAssignment) -> Result<(), YuhuhError> {
        Err(YuhuhError::NotImplemented)
    }
}
