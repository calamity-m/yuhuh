use async_trait::async_trait;
use chrono::NaiveDateTime;
use sqlx::PgPool;
use tracing::{error, info};
use uuid::Uuid;

use crate::{
    activity::model::{ActivityEntry, ActivityType},
    error::YuhuhError,
};

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
        if entries.is_empty() {
            error!("create_activity_entries received an empty vec");

            return Err(YuhuhError::BadRequest(
                "cannot create zero entries".to_string(),
            ));
        }

        let mut transaction = self.db.begin().await?;

        let mut user_id_vecs: Vec<Uuid> = vec![];
        let mut activity_vecs: Vec<String> = vec![];
        let mut activity_type: Vec<String> = vec![];
        let mut activity_info: Vec<serde_json::Value> = vec![];

        entries.into_iter().for_each(|f| {
            info!(activity_entry=?f, "added activity entry to creation query");
            activity_vecs.push(f.activity);
            activity_type.push(f.activity_type.to_string());
            activity_info.push(f.activity_info);
            user_id_vecs.push(f.user_id);
        });

        sqlx::query!(
            r#"
            INSERT INTO activity_records (
                user_id, 
                activity, 
                activity_type, 
                activity_info
            )
            SELECT * FROM UNNEST(
                $1::uuid[], 
                $2::text[],
                $3::text[],
                $4::jsonb[]
            )
            "#,
            &user_id_vecs[..],
            &activity_vecs[..],
            &activity_type[..],
            &activity_info[..]
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
