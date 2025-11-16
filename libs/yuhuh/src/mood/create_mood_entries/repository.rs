use async_trait::async_trait;
use sqlx::PgPool;
use tracing::{error, info};
use uuid::Uuid;

use crate::{error::YuhuhError, mood::model::MoodEntry};

// =============================================================================
// Traits
// =============================================================================

#[async_trait]
pub trait CreateMoodEntryRepository: std::fmt::Debug + Send + Sync + 'static {
    async fn create_mood_entries(&self, entries: Vec<MoodEntry>) -> Result<(), YuhuhError>;
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
    async fn create_mood_entries(&self, entries: Vec<MoodEntry>) -> Result<(), YuhuhError> {
        if entries.is_empty() {
            error!("create_mood_entries received an empty vec");

            return Err(YuhuhError::BadRequest(
                "cannot create zero entries".to_string(),
            ));
        }

        let mut transaction = self.db.begin().await?;

        let mut user_id_vecs: Vec<Uuid> = vec![];
        let mut mood_vecs: Vec<Option<i16>> = vec![];
        let mut energy_vecs: Vec<Option<i16>> = vec![];
        let mut sleep_vecs: Vec<Option<i16>> = vec![];
        let mut notes_vecs: Vec<Option<String>> = vec![];

        entries.into_iter().for_each(|m| {
            info!(mood_entrey=?m, "added mood entry to creation query");
            mood_vecs.push(m.mood.map(|mood| mood.get() as i16));
            energy_vecs.push(m.sleep.map(|sleep| sleep.get() as i16));
            sleep_vecs.push(m.energy.map(|energy| energy.get() as i16));
            user_id_vecs.push(m.user_id);
        });

        sqlx::query!(
            r#"
            INSERT INTO mood_records (
                user_id, 
                mood, 
                energy, 
                sleep,
                notes
            )
            SELECT * FROM UNNEST(
                $1::uuid[], 
                $2::smallint[],
                $3::smallint[],
                $4::smallint[],
                $5::text[]
            )
            "#,
            &user_id_vecs[..],
            &mood_vecs[..] as &[Option<i16>],
            &energy_vecs[..] as &[Option<i16>],
            &sleep_vecs[..] as &[Option<i16>],
            &notes_vecs[..] as &[Option<String>]
        )
        .execute(&mut *transaction)
        .await
        .map_err(|e| {
            error!(error = ?e, "database error while creating activity entries");

            YuhuhError::DatabaseError(e)
        })?;

        // Commit the transaction to persist all changes
        transaction.commit().await?;

        Ok(())
    }
}
