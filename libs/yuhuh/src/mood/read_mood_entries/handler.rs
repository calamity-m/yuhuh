//! read mood entries HTTP handler
//!
//! This module provides HTTP endpoints for reading mood entries.

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
    mood::{model::MoodEntry, state::MoodState},
    user::state::UserState,
};

// ============================================================================
// HTTP Request types
// ============================================================================

/// Request parameters for finding mood entries.
#[derive(Debug, Serialize, Deserialize, IntoParams)]
pub struct ReadMoodEntriesRequest {
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
pub struct ReadMoodEntriesResponse {
    pub found_mood_entries: u32,
    pub found_entries: Vec<MoodEntry>,
}

// =============================================================================
// HTTP Handlers
// =============================================================================

/// Find mood entries for a user
#[utoipa::path(
    get,
    path = "mood", 
    tag = "mood", 
    params(ReadMoodEntriesRequest),
    responses(
        (status = 200, description = "Found mood entries", body = ReadMoodEntriesResponse)
))]
#[instrument]
pub async fn read_mood_entries(
    State(mood_state): State<Arc<MoodState>>,
    State(user_state): State<Arc<UserState>>,
    Query(request): Query<ReadMoodEntriesRequest>,
) -> Result<(StatusCode, Json<ReadMoodEntriesResponse>), YuhuhError> {
    debug!("entering read_mood_entries");

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

    let records = mood_state
        .read_mood_entries_repo
        .find_mood_entries(
            &request.user_id,
            request.logged_before_date,
            request.logged_after_date,
            limit.into(),
            offset.into(),
        )
        .await?;

    Ok((
        StatusCode::OK,
        Json(ReadMoodEntriesResponse {
            found_mood_entries: records.len() as u32,
            found_entries: records,
        }),
    ))
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

    use crate::mood::read_mood_entries::ReadMoodEntriesResponse;

    #[tokio::test]
    async fn correct_user_mood_entries_returned() {
        let (app, db) = crate::test::common::setup().await;

        // Load test data into the database
        sqlx::raw_sql(include_str!("../../migrations/test/read_mood_entries.sql"))
            .execute(&db)
            .await
            .expect("setup test sql ran successfully");

        // Make a GET request to fetch food entries for the test user
        let response = app
            .oneshot(
                Request::builder()
                    .method("GET")
                    .uri("/mood?user_id=11111111-1111-1111-1111-111111111111")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        // Parse the response body
        let body = response.into_body().collect().await.unwrap().to_bytes();
        let mut dto: ReadMoodEntriesResponse =
            serde_json::from_slice(&body).expect("valid ReadMoodEntriesResponse bytes");

        // Check the overall length to ensure we got the correct ones
        assert_eq!(dto.found_entries.len(), 2);
        assert_eq!(dto.found_mood_entries, 2);

        // Check the first entry which should be the latest

        // Check mood/energy/sleep
        assert_eq!(
            &dto.found_entries[0]
                .mood
                .expect("mood should be populated")
                .get(),
            &10
        );
        assert_eq!(
            &dto.found_entries[0]
                .energy
                .expect("energy should be populated")
                .get(),
            &10
        );
        assert_eq!(&dto.found_entries[0].sleep, &None);

        // Check notes and updated now
        assert!(&dto.found_entries[0].updated_at.is_some());
        assert_eq!(
            dto.found_entries[0]
                .notes
                .as_mut()
                .expect("notes are not empty"),
            "alices mood thoughts"
        );

        // Now ensure the second entry is the older one
        assert_eq!(
            &dto.found_entries[1]
                .mood
                .expect("mood should be populated")
                .get(),
            &0
        );
        assert_eq!(
            &dto.found_entries[1]
                .energy
                .expect("energy should be populated")
                .get(),
            &1
        );
        assert_eq!(
            &dto.found_entries[1]
                .sleep
                .expect("sleep should be populated")
                .get(),
            &2
        );

        assert_eq!(&dto.found_entries[1].updated_at, &None);
        assert_eq!(&dto.found_entries[1].notes, &None);
    }

    #[tokio::test]
    async fn before_filters_correctly() {
        let (app, db) = crate::test::common::setup().await;

        // Load test data into the database
        sqlx::raw_sql(include_str!("../../migrations/test/read_mood_entries.sql"))
            .execute(&db)
            .await
            .expect("setup test sql ran successfully");

        // Build URI with current timestamp to filter out future entries
        let uri = format!(
            "/mood?user_id=11111111-1111-1111-1111-111111111111&logged_before_date={}",
            form_urlencoded::byte_serialize(Utc::now().to_rfc3339().as_bytes()).collect::<String>()
        );

        // Make a GET request to fetch food entries for the test user
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

        // Parse the response body
        let body = response.into_body().collect().await.unwrap().to_bytes();
        let dto: ReadMoodEntriesResponse =
            serde_json::from_slice(&body).expect("valid ReadMoodEntriesResponse bytes");

        assert_eq!(dto.found_entries.len(), 1);
        assert_eq!(dto.found_mood_entries, 1);
        assert_eq!(dto.found_entries[0].notes, None);
    }

    #[tokio::test]
    async fn after_filters_correctly() {
        let (app, db) = crate::test::common::setup().await;

        // Load test data into the database
        sqlx::raw_sql(include_str!("../../migrations/test/read_mood_entries.sql"))
            .execute(&db)
            .await
            .expect("setup test sql ran successfully");

        // Build URI with current timestamp to filter out future entries
        let uri = format!(
            "/mood?user_id=11111111-1111-1111-1111-111111111111&logged_after_date={}",
            form_urlencoded::byte_serialize(Utc::now().to_rfc3339().as_bytes()).collect::<String>()
        );

        // Make a GET request to fetch food entries for the test user
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

        // Parse the response body
        let body = response.into_body().collect().await.unwrap().to_bytes();
        let mut dto: ReadMoodEntriesResponse =
            serde_json::from_slice(&body).expect("valid ReadMoodEntriesResponse bytes");

        assert_eq!(dto.found_entries.len(), 1);
        assert_eq!(dto.found_mood_entries, 1);

        // We should have alice's mood thoughts, due to now() + interval '5 days'
        assert_eq!(
            dto.found_entries[0]
                .notes
                .as_mut()
                .expect("notes are populated"),
            "alices mood thoughts"
        );
    }

    #[tokio::test]
    async fn user_not_found_handled() {
        let (app, db) = crate::test::common::setup().await;

        // Load test data into the database
        sqlx::raw_sql(include_str!("../../migrations/test/read_mood_entries.sql"))
            .execute(&db)
            .await
            .expect("setup test sql ran successfully");

        // Request food entries for a user that doesn't exist in the database
        let response = app
            .oneshot(
                Request::builder()
                    .method("GET")
                    .uri("/mood?user_id=55555555-5555-5555-5555-555555555555")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn offset_paginates_correctly() {
        let (app, db) = crate::test::common::setup().await;

        // Load test data into the database
        sqlx::raw_sql(include_str!("../../migrations/test/read_mood_entries.sql"))
            .execute(&db)
            .await
            .expect("setup test sql ran successfully");

        // Request offset=1, limit=1 to get the 2nd entry (ordered descending by time)
        let response = app
            .oneshot(
                Request::builder()
                    .method("GET")
                    .uri("/mood?user_id=11111111-1111-1111-1111-111111111111&offset=1&limit=1")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        // Parse the response body
        let body = response.into_body().collect().await.unwrap().to_bytes();
        let dto: ReadMoodEntriesResponse =
            serde_json::from_slice(&body).expect("valid ReadMoodEntriesResponse bytes");

        // Should return only the third entry when skipping first two
        assert_eq!(dto.found_mood_entries, 1);

        // The no notes entry should be our found one since we are find the 2nd one
        assert_eq!(dto.found_entries[0].notes, None);
    }

    #[tokio::test]
    async fn limit_paginates_correctly() {
        let (app, db) = crate::test::common::setup().await;

        // Load test data into the database
        sqlx::raw_sql(include_str!("../../migrations/test/read_mood_entries.sql"))
            .execute(&db)
            .await
            .expect("setup test sql ran successfully");

        // use offset = 0 to fetch the first entry, due to the limit of 1
        let response = app
            .oneshot(
                Request::builder()
                    .method("GET")
                    .uri("/mood?user_id=11111111-1111-1111-1111-111111111111&offset=0&limit=1")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        // Parse the response body
        let body = response.into_body().collect().await.unwrap().to_bytes();
        let mut dto: ReadMoodEntriesResponse =
            serde_json::from_slice(&body).expect("valid ReadMoodEntriesResponse bytes");

        // Now we should have only the first entry
        assert_eq!(dto.found_entries.len(), 1);
        assert_eq!(dto.found_mood_entries, 1);
        assert_eq!(
            dto.found_entries[0]
                .notes
                .as_mut()
                .expect("notes are populated"),
            "alices mood thoughts"
        );
    }

    #[tokio::test]
    async fn empty_results_returns_correctly() {
        let (app, db) = crate::test::common::setup().await;

        // Load test data into the database
        sqlx::raw_sql(include_str!("../../migrations/test/read_mood_entries.sql"))
            .execute(&db)
            .await
            .expect("setup test sql ran successfully");

        // Make a GET request to fetch food entries for the test user
        let response = app
            .oneshot(
                Request::builder()
                    .method("GET")
                    .uri("/mood?user_id=22222222-2222-2222-2222-222222222222")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        // Parse the response body
        let body = response.into_body().collect().await.unwrap().to_bytes();
        let dto: ReadMoodEntriesResponse =
            serde_json::from_slice(&body).expect("valid ReadMoodEntriesResponse bytes");

        // Check the overall length to ensure we got the correct ones
        assert_eq!(dto.found_entries.len(), 0);
        assert_eq!(dto.found_mood_entries, 0);
    }
}
