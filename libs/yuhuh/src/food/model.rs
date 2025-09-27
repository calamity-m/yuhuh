use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct FoodEntry {
    pub food_record_id: Uuid,
    pub description: String,
    pub calories: Option<f32>,
    pub carbs: Option<f32>,
    pub protein: Option<f32>,
    pub fats: Option<f32>,
    pub micronutrients: Option<serde_json::Value>,
    pub user_id: Uuid,
    pub created_at: DateTime<Utc>,
}
