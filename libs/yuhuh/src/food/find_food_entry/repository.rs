use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::PgPool;
use tracing::{debug, error};
use uuid::Uuid;

use crate::{error::YuhuhError, food::model::FoodEntry};

#[async_trait]
pub trait FindFoodEntryRepository: std::fmt::Debug + Send + Sync + 'static {
    async fn find_food_entries(
        &self,
        user_id: &Uuid,
        before: Option<DateTime<Utc>>,
        after: Option<DateTime<Utc>>,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<FoodEntry>, YuhuhError>;
}

#[derive(Debug)]
pub struct FindFoodEntryRepositoryImpl {
    pub db: PgPool,
}

impl FindFoodEntryRepositoryImpl {
    pub fn new(db: PgPool) -> Self {
        FindFoodEntryRepositoryImpl { db }
    }
}

#[async_trait]
impl FindFoodEntryRepository for FindFoodEntryRepositoryImpl {
    async fn find_food_entries(
        &self,
        user_id: &Uuid,
        before: Option<DateTime<Utc>>,
        after: Option<DateTime<Utc>>,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<FoodEntry>, YuhuhError> {
        debug!(
            user_id=?user_id,
            before=?before,
            after=?after,
            limit=?limit,
            offset=?offset,
            "received find request for food entries"
        );

        let food_records: Vec<FoodEntry> = sqlx::query_as!(
            FoodEntry,
            r#"
            SELECT *
            FROM food_records
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
            error!(error = ?e, "database error while finding food records");

            YuhuhError::DatabaseError(e)
        })?;

        debug!(food_records=?food_records, "found food records");

        Ok(food_records)
    }
}
