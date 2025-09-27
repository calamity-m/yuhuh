use async_trait::async_trait;
use chrono::NaiveDateTime;
use sqlx::PgPool;
use tracing::{error, info};
use uuid::Uuid;

use crate::{error::YuhuhError, food::model::FoodEntry};

// =============================================================================
// Traits
// =============================================================================

#[async_trait]
pub trait CreateFoodEntryRepository: std::fmt::Debug + Send + Sync + 'static {
    async fn create_food_entries(&self, entries: Vec<FoodEntry>) -> Result<(), YuhuhError>;
}

// =============================================================================
// Production Implementation
// =============================================================================

#[derive(Debug)]
pub struct CreateFoodEntryRepositoryImpl {
    pub db: PgPool,
}

impl CreateFoodEntryRepositoryImpl {
    pub fn new(db: PgPool) -> Self {
        CreateFoodEntryRepositoryImpl { db }
    }
}

#[async_trait]
impl CreateFoodEntryRepository for CreateFoodEntryRepositoryImpl {
    async fn create_food_entries(&self, entries: Vec<FoodEntry>) -> Result<(), YuhuhError> {
        if entries.is_empty() {
            error!("create_food_entries received an empty vec");

            return Err(YuhuhError::BadRequest(
                "cannot create zero entries".to_string(),
            ));
        }

        let mut transaction = self.db.begin().await?;

        let mut description_vecs: Vec<String> = vec![];
        let mut calories_vecs: Vec<Option<f32>> = vec![];
        let mut carbs_vecs: Vec<Option<f32>> = vec![];
        let mut protein_vecs: Vec<Option<f32>> = vec![];
        let mut fats_vecs: Vec<Option<f32>> = vec![];
        let mut micronutrients_vecs: Vec<Option<serde_json::Value>> = vec![];
        let mut user_id_vecs: Vec<Uuid> = vec![];
        let mut created_at_vecs: Vec<NaiveDateTime> = vec![];

        entries.iter().for_each(|f| {
            info!(food_entry=?f, "added food entry to creation query");
            user_id_vecs.push(f.user_id);
            description_vecs.push(f.description.clone());
            calories_vecs.push(f.calories);
            carbs_vecs.push(f.carbs);
            protein_vecs.push(f.protein);
            fats_vecs.push(f.fats);
            micronutrients_vecs.push(f.micronutrients.clone());
            user_id_vecs.push(f.user_id);
            created_at_vecs.push(f.created_at.naive_utc());
        });

        sqlx::query!(
            r#"
            INSERT INTO food_records (
                user_id, 
                created_at, 
                description, 
                calories, 
                carbs, 
                protein, 
                fats, 
                micronutrients
            )
            SELECT * FROM UNNEST(
                $1::uuid[], 
                $2::timestamp[],
                $3::text[],
                $4::real[],
                $5::real[],
                $6::real[],
                $7::real[],
                $8::jsonb[]
            )
            "#,
            &user_id_vecs[..],
            &created_at_vecs[..],
            &description_vecs[..],
            &calories_vecs[..] as &[Option<f32>],
            &carbs_vecs[..] as &[Option<f32>],
            &protein_vecs[..] as &[Option<f32>],
            &fats_vecs[..] as &[Option<f32>],
            &micronutrients_vecs[..] as &[Option<serde_json::Value>],
        )
        .execute(&mut *transaction)
        .await?;

        // Commit the transaction to persist all changes
        transaction.commit().await?;

        Ok(())
    }
}
