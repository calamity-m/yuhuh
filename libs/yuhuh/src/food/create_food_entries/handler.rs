//! create food entry HTTP handler
//!
//! This module provides HTTP endpoints for creating food entries.

use std::sync::Arc;

use axum::{Json, extract::State, http::StatusCode};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tracing::{debug, instrument};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::{
    error::YuhuhError,
    food::{model::FoodEntry, state::FoodState},
    user::state::UserState,
};

// ============================================================================
// HTTP Request Types
// ============================================================================

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CreateFoodEntryRequest {
    pub user_id: Uuid,
    pub food_entries: Vec<NewFoodEntry>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct NewFoodEntry {
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

impl NewFoodEntry {
    pub fn into(&self, user_id: Uuid) -> FoodEntry {
        FoodEntry {
            food_record_id: None,
            user_id,
            description: self.description.clone(),
            calories: self.calories,
            carbs: self.carbs,
            protein: self.protein,
            fats: self.fats,
            micronutrients: self.micronutrients.clone(),
            created_at: Utc::now(),
            logged_at: self.logged_at.unwrap_or(Utc::now()),
        }
    }
}

// =============================================================================
// HTTP Handlers
// =============================================================================

/// Create food entries for a user
#[utoipa::path(
        post,
        path = "food/create",
        tag = "food",
        responses(
            (status = 201, description = "food entries created successfully"),
        )
    )]
#[instrument]
pub async fn create_food_entries(
    State(food_state): State<Arc<FoodState>>,
    State(user_state): State<Arc<UserState>>,
    Json(request): Json<CreateFoodEntryRequest>,
) -> Result<StatusCode, YuhuhError> {
    debug!("entering create_food_entries");

    if (user_state
        .find_user_repo
        .find_user_by_id(&request.user_id)
        .await?)
        .is_none()
    {
        return Err(YuhuhError::NotFound("user not found".to_string()));
    }

    let food_entries: Vec<FoodEntry> = request
        .food_entries
        .iter()
        .map(|f| f.into(request.user_id))
        .collect();

    debug!(food_entries=?food_entries, "food entries mapped");

    food_state
        .create_food_entries_repo
        .create_food_entries(food_entries)
        .await?;

    Ok(StatusCode::CREATED)
}

#[cfg(test)]
mod tests {

    use pretty_assertions::assert_eq;

    use axum::{
        body::Body,
        http::{self, Request, StatusCode},
    };
    use serde_json::json;
    use tower::ServiceExt;
    use uuid::uuid;

    use crate::food::create_food_entries::{CreateFoodEntryRequest, NewFoodEntry};

    #[tokio::test]
    async fn create_food_entries_correctly() {
        let (app, db) = crate::test::common::setup().await;

        // Load test data into the database
        sqlx::raw_sql(include_str!(
            "../../migrations/test/create_food_entries.sql"
        ))
        .execute(&db)
        .await
        .expect("setup test sql ran successfully");

        let request = CreateFoodEntryRequest {
            user_id: uuid!("11111111-1111-1111-1111-111111111111"),
            food_entries: vec![NewFoodEntry {
                description: "new food entry".to_string(),
                calories: Some(10.0),
                carbs: Some(10.0),
                protein: Some(10.0),
                fats: Some(10.0),
                micronutrients: Some(json!("{}")),
                logged_at: None,
            }],
        };

        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/food/create")
                    .header(http::header::CONTENT_TYPE, "application/json")
                    .body(Body::from(
                        serde_json::to_string(&request).expect("request is valid body"),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::CREATED);
    }

    #[tokio::test]
    async fn invalid_user_returns_not_found() {
        let (app, db) = crate::test::common::setup().await;

        // Load test data into the database
        sqlx::raw_sql(include_str!(
            "../../migrations/test/create_food_entries.sql"
        ))
        .execute(&db)
        .await
        .expect("setup test sql ran successfully");

        let request = CreateFoodEntryRequest {
            user_id: uuid!("11111111-5555-3333-2222-111111111111"),
            food_entries: vec![NewFoodEntry {
                description: "new food entry".to_string(),
                calories: Some(10.0),
                carbs: Some(10.0),
                protein: Some(10.0),
                fats: Some(10.0),
                micronutrients: Some(json!("{}")),
                logged_at: None,
            }],
        };

        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/food/create")
                    .header(http::header::CONTENT_TYPE, "application/json")
                    .body(Body::from(
                        serde_json::to_string(&request).expect("request is valid body"),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }
}
