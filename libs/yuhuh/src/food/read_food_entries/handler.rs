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
use tracing::{debug, error, instrument};
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;

use crate::{
    error::YuhuhError,
    food::{model::FoodEntry, state::FoodState},
    user::state::UserState,
};

// ============================================================================
// HTTP Request types
// ============================================================================

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

// ============================================================================
// HTTP Responsed types
// ============================================================================

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct FoundFoodRecord {
    pub description: String,
    pub calories: Option<f32>,
    pub carbs: Option<f32>,
    pub protein: Option<f32>,
    pub fats: Option<f32>,
    pub logged_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, ToSchema)]
pub struct CaloriesResult {
    pub total_calories: f32,
    pub food_entries_without_calories: u32,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, ToSchema)]
pub struct MacrosResult {
    pub total_carbs: f32,
    pub total_protein: f32,
    pub total_fats: f32,
    pub food_entries_without_carbs: u32,
    pub food_entries_without_protein: u32,
    pub food_entries_without_fats: u32,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct FindFoodEntryResponse {
    pub found_food_entries: u32,
    pub food_entries: Vec<FoundFoodRecord>,
    pub calories_result: CaloriesResult,
    pub macros_result: MacrosResult,
}

// ============================================================================
// Trait Implementations
// ============================================================================

impl From<&FoodEntry> for FoundFoodRecord {
    fn from(value: &FoodEntry) -> Self {
        FoundFoodRecord {
            description: value.description.clone(),
            calories: value.calories,
            carbs: value.carbs,
            protein: value.protein,
            fats: value.fats,
            logged_at: value.logged_at,
        }
    }
}

// =============================================================================
// HTTP Handlers
// =============================================================================

/// Find food entries for a user
#[utoipa::path(
    get,
    path = "food", 
    tag = "food", 
    params(FindFoodEntryRequest),
    responses(
        (status = 200, description = "Found food entries", body = FindFoodEntryResponse)
))]
#[instrument]
pub async fn read_food_entries(
    State(food_state): State<Arc<FoodState>>,
    State(user_state): State<Arc<UserState>>,

    Query(request): Query<FindFoodEntryRequest>,
) -> Result<(StatusCode, Json<FindFoodEntryResponse>), YuhuhError> {
    debug!("entering read_food_entries");

    if (user_state
        .find_user_repo
        .find_user_by_id(&request.user_id)
        .await?)
        .is_none()
    {
        error!(user_id = ?request.user_id, "failed to find user");
        return Err(YuhuhError::NotFound("user not found".to_string()));
    }

    let offset = request.offset.unwrap_or(0);
    let limit = request.limit.unwrap_or(10000);
    debug!(offset=?offset, limit=?limit, "calculated offset and limit");

    let food_records = food_state
        .read_food_entries_repo
        .read_food_entries(
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
    let mut mapped_food_records: Vec<FoundFoodRecord> = Vec::with_capacity(food_records.len());

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

        mapped_food_records.push(FoundFoodRecord::from(fr));
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

#[cfg(test)]
mod tests {

    use chrono::Utc;
    use http_body_util::BodyExt;
    use pretty_assertions::assert_eq;

    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use tower::ServiceExt;
    use url::form_urlencoded;

    use crate::food::read_food_entries::{CaloriesResult, FindFoodEntryResponse, MacrosResult};

    /// Tests that food entries are returned for a specific user in descending order by logged date
    #[tokio::test]
    async fn correct_user_food_entries_returned() {
        let (app, db, _) = crate::test::common::setup().await;

        // Load test data into the database
        sqlx::raw_sql(include_str!("../../migrations/test/read_food_entries.sql"))
            .execute(&db)
            .await
            .expect("setup test sql ran successfully");

        // Make a GET request to fetch food entries for the test user
        let response = app
            .oneshot(
                Request::builder()
                    .method("GET")
                    .uri("/food?user_id=11111111-1111-1111-1111-111111111111")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        // Parse the response body
        let body = response.into_body().collect().await.unwrap().to_bytes();
        let dto: FindFoodEntryResponse =
            serde_json::from_slice(&body).expect("valid FindFoodEntryResponse bytes");

        // Verify correct number of entries returned
        assert_eq!(dto.found_food_entries, 3);
        // Verify entries are sorted by date descending (newest first)
        assert_eq!(&dto.food_entries[0].description, "burger two"); // + 1 day
        assert_eq!(&dto.food_entries[1].description, "burger"); // - 1 day
        assert_eq!(&dto.food_entries[2].description, "burger three"); // - 5 days
    }

    /// Tests that total calories are calculated correctly from all food entries
    #[tokio::test]
    async fn calculates_calories_correctly() {
        let (app, db, _) = crate::test::common::setup().await;

        sqlx::raw_sql(include_str!("../../migrations/test/read_food_entries.sql"))
            .execute(&db)
            .await
            .expect("setup test sql ran successfully");

        let response = app
            .oneshot(
                Request::builder()
                    .method("GET")
                    .uri("/food?user_id=11111111-1111-1111-1111-111111111111")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = response.into_body().collect().await.unwrap().to_bytes();
        let dto: FindFoodEntryResponse =
            serde_json::from_slice(&body).expect("valid FindFoodEntryResponse bytes");

        // Verify calorie totals and count of entries missing calorie data
        assert_eq!(
            dto.calories_result,
            CaloriesResult {
                total_calories: 200.0,
                food_entries_without_calories: 1
            }
        );
    }

    /// Tests that macronutrients (carbs, protein, fats) are calculated correctly
    #[tokio::test]
    async fn calculates_macros_correctly() {
        let (app, db, _) = crate::test::common::setup().await;

        sqlx::raw_sql(include_str!("../../migrations/test/read_food_entries.sql"))
            .execute(&db)
            .await
            .expect("setup test sql ran successfully");

        let response = app
            .oneshot(
                Request::builder()
                    .method("GET")
                    .uri("/food?user_id=11111111-1111-1111-1111-111111111111")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = response.into_body().collect().await.unwrap().to_bytes();
        let dto: FindFoodEntryResponse =
            serde_json::from_slice(&body).expect("valid FindFoodEntryResponse bytes");

        // Verify macro totals and count of entries missing each macro
        assert_eq!(
            dto.macros_result,
            MacrosResult {
                total_carbs: 10.0,
                total_protein: 10.0,
                total_fats: 10.0,
                food_entries_without_carbs: 1,
                food_entries_without_protein: 1,
                food_entries_without_fats: 1
            }
        );
    }

    /// Tests that the logged_before_date filter correctly excludes entries after the specified date
    #[tokio::test]
    async fn before_filters_correctly() {
        let (app, db, _) = crate::test::common::setup().await;

        sqlx::raw_sql(include_str!("../../migrations/test/read_food_entries.sql"))
            .execute(&db)
            .await
            .expect("setup test sql ran successfully");

        // Build URI with current timestamp to filter out future entries
        let uri = format!(
            "/food?user_id=11111111-1111-1111-1111-111111111111&logged_before_date={}",
            form_urlencoded::byte_serialize(Utc::now().to_rfc3339().as_bytes()).collect::<String>()
        );

        println!("{}", uri);
        let response = app
            .oneshot(
                Request::builder()
                    .method("GET")
                    .uri(uri)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = response.into_body().collect().await.unwrap().to_bytes();
        let dto: FindFoodEntryResponse =
            serde_json::from_slice(&body).expect("valid FindFoodEntryResponse bytes");

        // Should only return entries before current time (excludes future entry)
        assert_eq!(dto.found_food_entries, 2);
        assert_eq!(&dto.food_entries[0].description, "burger");
        assert_eq!(&dto.food_entries[1].description, "burger three");
    }

    /// Tests that the logged_after_date filter correctly excludes entries before the specified date
    #[tokio::test]
    async fn after_filters_correctly() {
        let (app, db, _) = crate::test::common::setup().await;

        sqlx::raw_sql(include_str!("../../migrations/test/read_food_entries.sql"))
            .execute(&db)
            .await
            .expect("setup test sql ran successfully");

        // Build URI with current timestamp to filter out past entries
        let uri = format!(
            "/food?user_id=11111111-1111-1111-1111-111111111111&logged_after_date={}",
            form_urlencoded::byte_serialize(Utc::now().to_rfc3339().as_bytes()).collect::<String>()
        );

        println!("{}", uri);
        let response = app
            .oneshot(
                Request::builder()
                    .method("GET")
                    .uri(uri)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = response.into_body().collect().await.unwrap().to_bytes();
        let dto: FindFoodEntryResponse =
            serde_json::from_slice(&body).expect("valid FindFoodEntryResponse bytes");

        // Should only return entries after current time (only the future entry)
        assert_eq!(dto.found_food_entries, 1);
        assert_eq!(&dto.food_entries[0].description, "burger two");
    }

    /// Tests that pagination with offset and limit works correctly
    #[tokio::test]
    async fn paginates_correctly() {
        let (app, db, _) = crate::test::common::setup().await;

        sqlx::raw_sql(include_str!("../../migrations/test/read_food_entries.sql"))
            .execute(&db)
            .await
            .expect("setup test sql ran successfully");

        // Request offset=2, limit=1 to get the third entry (ordered descending by time)
        let response = app
            .oneshot(
                Request::builder()
                    .method("GET")
                    .uri("/food?user_id=11111111-1111-1111-1111-111111111111&offset=2&limit=1")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = response.into_body().collect().await.unwrap().to_bytes();
        let dto: FindFoodEntryResponse =
            serde_json::from_slice(&body).expect("valid FindFoodEntryResponse bytes");

        // Should return only the third entry when skipping first two
        assert_eq!(dto.found_food_entries, 1);
        assert_eq!(&dto.food_entries[0].description, "burger three");
    }

    /// Tests that querying for a non-existent user returns 404 Not Found
    #[tokio::test]
    async fn user_not_found_handled() {
        let (app, db, _) = crate::test::common::setup().await;

        sqlx::raw_sql(include_str!("../../migrations/test/read_food_entries.sql"))
            .execute(&db)
            .await
            .expect("setup test sql ran successfully");

        // Request food entries for a user that doesn't exist in the database
        let response = app
            .oneshot(
                Request::builder()
                    .method("GET")
                    .uri("/food?user_id=55555555-5555-5555-5555-555555555555")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }
}
