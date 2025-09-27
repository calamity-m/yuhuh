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
#[derive(Debug, Serialize, ToSchema)]
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

/// Handles a request to find a user by ID or Discord ID.
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
    tag = "find user", 
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
