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
use tracing::instrument;
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;

use crate::{error::YuhuhError, food::state::FoodState};

/// Request parameters for finding a user.
#[derive(Debug, Deserialize, IntoParams)]
pub struct FindFoodEntryRequest {
    /// user ID to search by.
    pub id: Uuid,
    pub limit: Option<u32>,
    pub offset: Option<u32>,
    pub logged_before_date: Option<DateTime<Utc>>,
    pub logged_before_after: Option<DateTime<Utc>>,
    pub calculate_calories: Option<bool>,
    pub calculate_macros: Option<bool>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct FoodEntryRecord {
    pub descriptipn: String,
    pub calories: f32,
    pub carbs: f32,
    pub protein: f32,
    pub fats: f32,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct CaloriesResult {
    pub total_calories: u64,
    pub food_entries_without_calories: u32,
    pub confidence_rating: f32,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct MacrosResult {
    pub total_carbs: u64,
    pub total_protein: u64,
    pub total_fats: u64,
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

    Err(YuhuhError::NotImplemented)

}