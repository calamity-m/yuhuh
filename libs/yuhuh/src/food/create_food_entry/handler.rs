//! create food entry HTTP handler
//!
//! This module provides HTTP endpoints for creating food entries.

use std::sync::Arc;

use axum::{Json, extract::State, http::StatusCode};
use chrono::{DateTime, Utc};
use serde::Deserialize;
use tracing::{debug, instrument};
use tracing_subscriber::field::debug;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::{
    error::YuhuhError,
    food::{
        model::{FoodEntry, NewFoodEntry},
        state::FoodState,
    },
    user,
};

// ============================================================================
// Request/Response Types
// ============================================================================

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateFoodEntryRequest {
    pub user_id: Uuid,
    pub food_entries: Vec<FoodEntryRequest>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct FoodEntryRequest {
    pub description: String,
    pub calories: Option<f32>,
    pub carbs: Option<f32>,
    pub protein: Option<f32>,
    pub fats: Option<f32>,
    pub micronutrients: Option<serde_json::Value>,
    pub logged_at: Option<DateTime<Utc>>,
}

// ============================================================================
// Implementations
// ============================================================================

impl FoodEntryRequest {
    pub fn into_new_food_entry(&self, user_id: Uuid) -> NewFoodEntry {
        NewFoodEntry {
            user_id,
            description: self.description.clone(),
            calories: self.calories,
            carbs: self.carbs,
            protein: self.protein,
            fats: self.fats,
            micronutrients: self.micronutrients.clone(),
            created_at: self.logged_at.unwrap_or(Utc::now()),
        }
    }
}

// =============================================================================
// HTTP Handlers
// =============================================================================

/// Creates a new food entry with associated user data.
#[utoipa::path(
        post,
        path = "food/create",
        tag = "create food entry",
        responses(
            (status = 201, description = "food entry created successfully"),
        )
    )]
#[axum::debug_handler]
#[instrument]
pub async fn create_food_entry(
    State(food_state): State<Arc<FoodState>>,
    Json(request): Json<CreateFoodEntryRequest>,
) -> Result<StatusCode, YuhuhError> {
    debug!("entering create_food_entry");

    let food_entries: Vec<NewFoodEntry> = request
        .food_entries
        .iter()
        .map(|f| f.into_new_food_entry(request.user_id))
        .collect();

    debug!(food_entries=?food_entries, "food entries mapped");

    food_state
        .create_food_entry_repo
        .create_food_entries(food_entries)
        .await?;

    Ok(StatusCode::CREATED)
}
