//! User search handlers and related types.

use axum::{
    Json,
    extract::{Query, State},
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tracing::{debug, error, info, instrument};
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;

use crate::{
    error::YuhuhError,
    user::{model::User, state::UserState},
};

// ============================================================================
// Request/Response Types
// ============================================================================

/// Request parameters for finding a user.
#[derive(Debug, Deserialize, IntoParams)]
pub struct FindUserRequest {
    /// Optional user ID to search by.
    pub id: Option<Uuid>,
    /// Optional Discord ID to search by.
    pub discord_id: Option<i64>,
}

/// Response containing user information.
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct FindUserResponse {
    pub id: Uuid,
    pub personalisation: Option<String>,
    pub contact_email: Option<String>,
    pub contact_name: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
    pub timezone: Option<String>,
    pub discord_id: Option<i64>,
    pub discord_username: Option<String>,
}

// ============================================================================
// Trait Implementations
// ============================================================================

impl From<User> for FindUserResponse {
    fn from(user: User) -> Self {
        let (discord_id, discord_username) = user
            .discord_user
            .map(|d| (Some(d.discord_id), Some(d.username)))
            .unwrap_or((None, None));

        Self {
            id: user.user_id,
            personalisation: user.personalisation.to_owned(),
            contact_email: user.contact_email,
            contact_name: user.contact_name,
            created_at: user.created_at,
            updated_at: user.updated_at,
            timezone: user.timezone,
            discord_id,
            discord_username,
        }
    }
}

// =============================================================================
// HTTP Handlers
// =============================================================================

/// Find users.
///
/// This endpoint extracts query parameters and application state,
/// attempts to look up the user in the repository, and returns
/// the user as JSON if found.
///
/// # Arguments
/// * `request` - Query parameters containing search criteria
/// * `user_state` - Application state containing user repository
///
/// # Returns
/// * `Ok(Json<FindUserResponse>)` - User found and returned as JSON
/// * `Err(YuhuhError::NotFound)` - If no user exists with the given ID
/// * `Err(YuhuhError)` - Error occurred during lookup or validation
///
#[utoipa::path(
    get,
    path = "users", 
    tag = "users", 
    params(FindUserRequest),
    responses(
        (status = 200, description = "Found user", body = FindUserResponse)
))]
#[instrument]
pub async fn find_user(
    State(user_state): State<Arc<UserState>>,
    Query(request): Query<FindUserRequest>,
) -> Result<Json<FindUserResponse>, YuhuhError> {
    debug!("entered find_user - request: {:?}", request);

    // Handle user ID search
    if let Some(id) = request.id {
        let result = user_state
            .find_user_repo
            .find_user_by_id(&id)
            .await?
            .ok_or_else(|| YuhuhError::NotFound("user not found from user uuid id".to_string()))
            .map(|u| Json(FindUserResponse::from(u)));

        match &result {
            Ok(user) => info!(user = ?user, "found user"),
            Err(err) => error!(error = ?err, "failed to find user"),
        }

        return result;
    }

    // Handle Discord ID search (not yet implemented)
    if let Some(discord_id) = request.discord_id {
        let result = user_state
            .find_user_repo
            .find_user_by_discord_id(discord_id)
            .await?
            .ok_or_else(|| YuhuhError::NotFound("user not found from discord id".to_string()))
            .map(|u| Json(FindUserResponse::from(u)));

        match &result {
            Ok(user) => info!(user = ?user, "found user"),
            Err(err) => error!(error = ?err, "failed to find user"),
        }

        return result;
    }

    // No valid query parameters provided
    Err(YuhuhError::BadRequest("missing query".to_string()))
}

#[cfg(test)]
mod tests {
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use chrono::{DateTime, Utc};
    use pretty_assertions::assert_eq;
    use tower::ServiceExt;
    use uuid::uuid;

    use crate::user::find_user::FindUserResponse;
    use http_body_util::BodyExt;

    #[tokio::test]
    async fn correct_user_returned_from_id() {
        let (app, db) = crate::test::common::setup().await;

        // Load test data into the database
        sqlx::raw_sql(include_str!("../../migrations/test/find_user.sql"))
            .execute(&db)
            .await
            .expect("setup test sql ran successfully");

        let response = app
            .oneshot(
                Request::builder()
                    .method("GET")
                    .uri("/users?id=22222222-2222-2222-2222-222222222222")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body = response.into_body().collect().await.unwrap().to_bytes();
        let dto: FindUserResponse =
            serde_json::from_slice(&body).expect("valid FindUserResponse bytes");

        let created_at: DateTime<Utc> = "1999-09-09T09:09:09Z".parse().unwrap();
        let updated_at: DateTime<Utc> = "2000-09-09T09:09:09Z".parse().unwrap();

        assert_eq!(dto.id, uuid!("22222222-2222-2222-2222-222222222222"));
        assert_eq!(dto.discord_id, None);
        assert_eq!(dto.discord_username, None);
        assert_eq!(dto.personalisation, Some("personalized".to_string()));
        assert_eq!(dto.contact_email, Some("bobat@example.com".to_string()));
        assert_eq!(dto.contact_name, Some("Bobat".to_string()));
        assert_eq!(dto.created_at, created_at);
        assert_eq!(dto.updated_at, Some(updated_at));
    }

    #[tokio::test]
    async fn correct_user_returned_from_discord_id() {
        let (app, db) = crate::test::common::setup().await;

        // Load test data into the database
        sqlx::raw_sql(include_str!("../../migrations/test/find_user.sql"))
            .execute(&db)
            .await
            .expect("setup test sql ran successfully");

        let response = app
            .oneshot(
                Request::builder()
                    .method("GET")
                    .uri("/users?discord_id=100")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body = response.into_body().collect().await.unwrap().to_bytes();
        let dto: FindUserResponse =
            serde_json::from_slice(&body).expect("valid FindUserResponse bytes");

        assert_eq!(dto.discord_id, Some(100));
        assert_eq!(dto.discord_username, Some("alicediscord".to_string()));
        assert_eq!(dto.id, uuid!("11111111-1111-1111-1111-111111111111"));
        assert_eq!(dto.contact_name, Some("Alice".to_string()))
    }
}
