use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    error::YuhuhError,
    mood::model::{EnergyAssignment, Rating, MoodAssignment, SleepAssignment},
};

// =============================================================================
// Traits
// =============================================================================

#[async_trait]
pub trait ReadAssignmentRepository: std::fmt::Debug + Send + Sync + 'static {
    async fn read_mood_assigment_index(
        &self,
        user_id: &Uuid,
        index: Rating,
    ) -> Result<Option<MoodAssignment>, YuhuhError>;

    async fn read_energy_assigment_index(
        &self,
        user_id: &Uuid,
        index: Rating,
    ) -> Result<Option<EnergyAssignment>, YuhuhError>;

    async fn read_sleep_assigment_index(
        &self,
        user_id: &Uuid,
        index: Rating,
    ) -> Result<Option<SleepAssignment>, YuhuhError>;

    async fn read_mood_assignments(
        &self,
        user_id: &Uuid,
    ) -> Result<Vec<MoodAssignment>, YuhuhError>;

    async fn read_energy_assignments(
        &self,
        user_id: &Uuid,
    ) -> Result<Vec<EnergyAssignment>, YuhuhError>;

    async fn read_sleep_assignments(
        &self,
        user_id: &Uuid,
    ) -> Result<Vec<SleepAssignment>, YuhuhError>;
}

// =============================================================================
// Production Implementation
// =============================================================================

#[derive(Debug)]
pub struct ReadAssignmentRepositoryImpl {
    pub db: PgPool,
}

impl ReadAssignmentRepositoryImpl {
    pub fn new(db: PgPool) -> Self {
        ReadAssignmentRepositoryImpl { db }
    }
}

#[async_trait]
impl ReadAssignmentRepository for ReadAssignmentRepositoryImpl {
    async fn read_mood_assigment_index(
        &self,
        user_id: &Uuid,
        index: Rating,
    ) -> Result<Option<MoodAssignment>, YuhuhError> {
        Err(YuhuhError::NotImplemented)
    }

    async fn read_energy_assigment_index(
        &self,
        user_id: &Uuid,
        index: Rating,
    ) -> Result<Option<EnergyAssignment>, YuhuhError> {
        Err(YuhuhError::NotImplemented)
    }

    async fn read_sleep_assigment_index(
        &self,
        user_id: &Uuid,
        index: Rating,
    ) -> Result<Option<SleepAssignment>, YuhuhError> {
        Err(YuhuhError::NotImplemented)
    }

    async fn read_mood_assignments(
        &self,
        user_id: &Uuid,
    ) -> Result<Vec<MoodAssignment>, YuhuhError> {
        Err(YuhuhError::NotImplemented)
    }

    async fn read_energy_assignments(
        &self,
        user_id: &Uuid,
    ) -> Result<Vec<EnergyAssignment>, YuhuhError> {
        Err(YuhuhError::NotImplemented)
    }

    async fn read_sleep_assignments(
        &self,
        user_id: &Uuid,
    ) -> Result<Vec<SleepAssignment>, YuhuhError> {
        Err(YuhuhError::NotImplemented)
    }
}
