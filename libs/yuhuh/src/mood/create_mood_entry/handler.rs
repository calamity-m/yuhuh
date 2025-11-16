//! create mood entries HTTP handler
//!
//! This module provides HTTP endpoints for creating mood entries.

// ============================================================================
// Request/Response Types
// ============================================================================

use std::sync::Arc;

use axum::{Json, extract::State, http::StatusCode};

use serde::{Deserialize, Serialize};
use tracing::{debug, error, instrument};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::{
    error::YuhuhError,
    mood::{model::MoodEntry, rating::Rating, state::MoodState},
    user::state::UserState,
};

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CreateMoodEntryRequest {
    pub user_id: Uuid,
    pub notes: Option<String>,
    pub mood: Option<Rating>,
    pub energy: Option<Rating>,
    pub sleep: Option<Rating>,
}

// ============================================================================
// HTTP Request types
// ============================================================================

/// Create mood entry for a user
#[utoipa::path(
    post,
    path = "mood", 
    tag = "mood", 
    responses(
        (status = 201, description = "mood created successfully")
))]
#[instrument]
pub async fn create_mood_entry(
    State(mood_state): State<Arc<MoodState>>,
    State(user_state): State<Arc<UserState>>,
    Json(request): Json<CreateMoodEntryRequest>,
) -> Result<StatusCode, YuhuhError> {
    if (user_state
        .find_user_repo
        .find_user_by_id(&request.user_id)
        .await?)
        .is_none()
    {
        error!(user_id = ?request.user_id, "failed to find user");
        return Err(YuhuhError::NotFound("user not found".to_string()));
    }

    let m = MoodEntry {
        mood_record_id: None,
        user_id: request.user_id,
        created_at: None,
        updated_at: None,
        mood: request.mood,
        energy: request.energy,
        sleep: request.sleep,
        notes: request.notes,
    };

    debug!(entry=?m, "creating mood entry");

    mood_state
        .create_mood_entry_repo
        .create_mood_entry(m)
        .await?;

    debug!("successfully created mood entry");

    Ok(StatusCode::CREATED)
}

#[cfg(test)]
mod tests {

    use pretty_assertions::assert_eq;

    use axum::{
        body::Body,
        http::{self, Request, StatusCode},
    };
    use tower::ServiceExt;
    use uuid::uuid;

    use crate::mood::{create_mood_entry::CreateMoodEntryRequest, rating::Rating};

    #[tokio::test]
    async fn create_mood_entry_correctly() {
        let (app, db) = crate::test::common::setup().await;

        // Load test data into the database
        sqlx::raw_sql(include_str!("../../migrations/test/create_mood_entry.sql"))
            .execute(&db)
            .await
            .expect("setup test sql ran successfully");

        let request = CreateMoodEntryRequest {
            user_id: uuid!("11111111-1111-1111-1111-111111111111"),
            notes: Some("notes".to_string()),
            mood: Rating::new(1),
            energy: Rating::new(1),
            sleep: Rating::new(1),
        };

        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/mood")
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
        sqlx::raw_sql(include_str!("../../migrations/test/create_mood_entry.sql"))
            .execute(&db)
            .await
            .expect("setup test sql ran successfully");

        let request = CreateMoodEntryRequest {
            user_id: uuid!("11111111-5555-3333-2222-111111111111"),
            notes: Some("notes".to_string()),
            mood: Rating::new(1),
            energy: Rating::new(1),
            sleep: Rating::new(1),
        };

        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/mood")
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
