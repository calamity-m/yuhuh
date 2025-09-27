use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{error::YuhuhError, food::model::FoodEntry};

#[async_trait]
pub trait FindFoodEntryRepository: std::fmt::Debug + Send + Sync + 'static {
    async fn find_food_entries(
        &self,
        user_id: &Uuid,
        before: DateTime<Utc>,
        after: DateTime<Utc>,
        limit: Option<u32>,
        offset: Option<u32>,
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
        before: DateTime<Utc>,
        after: DateTime<Utc>,
        limit: Option<u32>,
        offset: Option<u32>,
    ) -> Result<Vec<FoodEntry>, YuhuhError> {
        Err(YuhuhError::NotImplemented)
    }
}
