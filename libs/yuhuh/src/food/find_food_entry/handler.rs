//! create food entry HTTP handler
//!
//! This module provides HTTP endpoints for creating food entries.

// ============================================================================
// Request/Response Types
// ============================================================================

use std::sync::Arc;

use axum::{
    Json,
    extract::{Query, State},
    http::StatusCode,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tracing::{debug, instrument};
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;

use crate::{
    error::YuhuhError,
    food::{model::FoodEntry, state::FoodState},
};

/// Request parameters for finding a user.
#[derive(Debug, Deserialize, IntoParams)]
pub struct FindFoodEntryRequest {
    /// user ID to search by.
    pub user_id: Uuid,
    pub limit: Option<u32>,
    pub offset: Option<u32>,
    pub logged_before_date: Option<DateTime<Utc>>,
    pub logged_after_date: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct FoodEntryRecord {
    pub description: String,
    pub calories: Option<f32>,
    pub carbs: Option<f32>,
    pub protein: Option<f32>,
    pub fats: Option<f32>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct CaloriesResult {
    pub total_calories: f32,
    pub food_entries_without_calories: u32,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct MacrosResult {
    pub total_carbs: f32,
    pub total_protein: f32,
    pub total_fats: f32,
    pub food_entries_without_carbs: u32,
    pub food_entries_without_protein: u32,
    pub food_entries_without_fats: u32,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct FindFoodEntryResponse {
    pub found_food_entries: u32,
    pub food_entries: Vec<FoodEntryRecord>,
    pub calories_result: CaloriesResult,
    pub macros_result: MacrosResult,
}

// ============================================================================
// Trait Implementations
// ============================================================================

impl From<&FoodEntry> for FoodEntryRecord {
    fn from(value: &FoodEntry) -> Self {
        FoodEntryRecord {
            description: value.description.clone(),
            calories: value.calories,
            carbs: value.carbs,
            protein: value.protein,
            fats: value.fats,
        }
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
    debug!("entering find_food_entry");

    let offset = request.offset.unwrap_or(0);
    let limit = request.limit.unwrap_or(10000);
    debug!(offset=?offset, limit=?limit, "calculated offset and limit");

    let food_records = food_state
        .find_food_entry_repo
        .find_food_entries(
            &request.user_id,
            request.logged_before_date,
            request.logged_after_date,
            limit.into(),
            offset.into(),
        )
        .await?;

    let mut calories_result = CaloriesResult {
        total_calories: 0.0,
        food_entries_without_calories: 0,
    };
    let mut macros_result = MacrosResult {
        total_carbs: 0.0,
        total_protein: 0.0,
        total_fats: 0.0,
        food_entries_without_carbs: 0,
        food_entries_without_protein: 0,
        food_entries_without_fats: 0,
    };
    let mut mapped_food_records: Vec<FoodEntryRecord> = Vec::with_capacity(food_records.len());

    food_records.iter().for_each(|fr| {
        if let Some(calories) = fr.calories {
            calories_result.total_calories += calories;
        } else {
            calories_result.food_entries_without_calories += 1;
        }

        if let Some(carbs) = fr.carbs {
            macros_result.total_carbs += carbs;
        } else {
            macros_result.food_entries_without_carbs += 1;
        }

        if let Some(protein) = fr.protein {
            macros_result.total_protein += protein;
        } else {
            macros_result.food_entries_without_protein += 1;
        }

        if let Some(fats) = fr.fats {
            macros_result.total_fats += fats;
        } else {
            macros_result.food_entries_without_fats += 1;
        }

        mapped_food_records.push(FoodEntryRecord::from(fr));
    });

    let response = Json(FindFoodEntryResponse {
        found_food_entries: mapped_food_records.len() as u32,
        food_entries: mapped_food_records,
        calories_result,
        macros_result,
    });

    debug!(response=?response, "found food records");

    Ok((StatusCode::OK, response))
}
