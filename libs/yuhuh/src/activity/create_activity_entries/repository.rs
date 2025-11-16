use async_trait::async_trait;
use sqlx::PgPool;

use crate::{activity::model::ActivityEntry, error::YuhuhError};

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
            error!("create_food_entries received an empty vec");

            return Err(YuhuhError::BadRequest(
                "cannot create zero entries".to_string(),
            ));
        }

        let mut transaction = self.db.begin().await?;

        let mut user_id_vecs: Vec<Uuid> = vec![];
        let mut activity_vecs: Vec<String> = vec![];
        let mut activity_type: Vec<ActivityType> = vec![];
        let mut activity_info: Vec<serde_json::Value> = vec![];

        entries.iter().for_each(|f| {
            info!(food_entry=?f, "added food entry to creation query");
            activity_vecs.push(f.activity.clone());
            activity_type.push(f.activity_type);
            activity_info.push(f.activity_info);
            user_id_vecs.push(f.user_id);
        });

        sqlx::query!(
            r#"
            INSERT INTO food_records (
                user_id, 
                activity, 
                activity_type, 
                activity_info
            )
            SELECT * FROM UNNEST(
                $1::uuid[], 
                $2::text[],
                $3::text[],
                $4::jsonb[],
            )
            "#,
            &user_id_vecs[..],
            &activity_vecs[..],
            &activity_type[..],
            &activity_info[..],
        )
        .execute(&mut *transaction)
        .await
        .map_err(|e| {
            error!(error = ?e, "database error while creating food entries");

            YuhuhError::DatabaseError(e)
        })?;

        // Commit the transaction to persist all changes
        transaction.commit().await?;

        Ok(())
    }
}
