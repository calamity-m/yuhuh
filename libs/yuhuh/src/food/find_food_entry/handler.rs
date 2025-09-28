//! create food entry HTTP handler
//!
//! This module provides HTTP endpoints for creating food entries.

// ============================================================================
// Request/Response Types
// ============================================================================

use std::sync::Arc;

use axum::{extract::{Query, State}, http::StatusCode, Json};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tracing::{debug, instrument};
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;

use crate::{error::YuhuhError, food::{model::FoodEntry, state::FoodState}};

/// Request parameters for finding a user.
#[derive(Debug, Deserialize, IntoParams)]
pub struct FindFoodEntryRequest {
    /// user ID to search by.
    pub user_id: Uuid,
    pub limit: Option<u32>,
    pub offset: Option<u32>,
    pub logged_before_date: Option<DateTime<Utc>>,
    pub logged_after_date: Option<DateTime<Utc>>,
    pub calculate_calories: Option<bool>,
    pub calculate_macros: Option<bool>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct FoodEntryRecord {
    pub descriptipn: String,
    pub calories: Option<f32>,
    pub carbs: Option<f32>,
    pub protein: Option<f32>,
    pub fats: Option<f32>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct CaloriesResult {
    pub total_calories: f32,
    pub food_entries_without_calories: u32,
    pub confidence_rating: f32,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct MacrosResult {
    pub total_carbs: f32,
    pub total_protein: f32,
    pub total_fats: f32,
    pub food_entries_without_carbs: u32,
    pub food_entries_without_protein: u32,
    pub food_entries_without_fats: u32,
    pub confidence_rating: f32,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct FindFoodEntryResponse {
    pub found_food_entries: u32,
    pub food_entries: Vec<FoodEntryRecord>,
    pub calories_result: Option<CaloriesResult>,
    pub macros_result: Option<MacrosResult>,
}

// ============================================================================
// Trait Implementations
// ============================================================================

impl From<&FoodEntry> for FoodEntryRecord {
    fn from(value: &FoodEntry) -> Self {
        FoodEntryRecord { descriptipn: value.description.clone(), calories: value.calories, carbs: value.carbs, protein: value.protein, fats: value.fats }
    }
}

// =============================================================================
// HTTP Handlers
// =============================================================================

#[utoipa::path(
    get, 
    path = "food", 
    tag = "find food", 
    params(FindFoodEntryRequest),
    responses(
        (status = 200, description = "Found food entries", body = FindFoodEntryResponse)
))]
#[instrument]
pub async fn find_food_entry(
    State(food_state): State<Arc<FoodState>>,
    Query(request): Query<FindFoodEntryRequest>,
) -> Result<(StatusCode, Json<FindFoodEntryResponse>), YuhuhError> {

    let offset = request.offset.unwrap_or(0);
    let limit = request.limit.unwrap_or(10000);
    debug!(offset=?offset, limit=?limit, "calculated offset and limit");

    let food_records = food_state.find_food_entry_repo.find_food_entries(
        &request.user_id, 
        request.logged_before_date, 
        request.logged_after_date, 
        limit.into(), 
        offset.into()).await?;

        // This needs to change and calculated on one iteration, rather than 3... one for calories, then one for macros and last for mapping to the dto record lol
    let calculated_calories = match request.calculate_calories {
        Some(true) => Some(CaloriesResult { total_calories: 0.0, food_entries_without_calories: 0, confidence_rating: 0.0 }),
        Some(false) => None,
        None => None,
    };

    let calculated_macros = match request.calculate_macros {
        Some(true) => Some(MacrosResult {total_carbs:0.0,total_fats:0.0,total_protein:0.0, food_entries_without_carbs: 0, food_entries_without_protein: 0, food_entries_without_fats: 0, confidence_rating: 0.0 }),
        Some(false) => None,
        None => None,
    };

    debug!(food_records=?food_records, "found records");

    let r = Json(FindFoodEntryResponse{ found_food_entries: 0, food_entries: food_records.iter().map(FoodEntryRecord::from).collect(), calories_result: calculated_calories, macros_result: calculated_macros });

    let rr = (StatusCode::OK, r);


    Ok(rr)

}