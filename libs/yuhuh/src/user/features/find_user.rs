//! User search handlers and related types.

use axum::{
    Json,
    extract::{Query, State},
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::sync::Arc;
use tracing::{debug, error, info, instrument};
use uuid::Uuid;

use crate::{
    error::YuhuhError,
    user::{model::User, state::UserState},
};

// ============================================================================
// Request/Response Types
// ============================================================================

/// Request parameters for finding a user.
#[derive(Debug, Deserialize)]
pub struct FindUserRequest {
    /// Optional user ID to search by.
    id: Option<Uuid>,
    /// Optional Discord ID to search by.
    discord_id: Option<i64>,
}

/// Response containing user information.
#[derive(Debug, Serialize)]
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

// ============================================================================
// Handler Functions
// ============================================================================

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
#[instrument]
pub async fn handler(
    Query(request): Query<FindUserRequest>,
    State(user_state): State<Arc<UserState>>,
) -> Result<Json<FindUserResponse>, YuhuhError> {
    debug!("entered find_user - request: {:?}", request);

    // Handle user ID search
    if let Some(id) = request.id {
        let user: User = find_user_by_id(&user_state.db, &id).await?;

        info!(user = ?user, "found user");
        return Ok(Json(FindUserResponse::from(user)));
    }

    // Handle Discord ID search (not yet implemented)
    if let Some(discord_id) = request.discord_id {
        let user: User = find_user_by_discord_id(&user_state.db, discord_id).await?;

        info!(user = ?user, "found user based on discord id {}", discord_id);
        return Ok(Json(FindUserResponse::from(user)));
    }

    // No valid query parameters provided
    Err(YuhuhError::BadRequest("missing query".to_string()))
}

// ============================================================================
// Query Functions
// ============================================================================

/// Queries the database for a user based on their ID. Returns a user joined
/// with their discord_user if present.
///
/// # Arguments
/// * `db` - PostgreSQL connection pool for database access
/// * `id` - UUID of the user to find
///
/// # Returns
/// * `Ok(User)` - The user record if found, with joined discord_user data if available
/// * `Err(YuhuhError::NotFound)` - If no user exists with the given ID
/// * `Err(YuhuhError::ContextError)` - If a database error occurs during the query
async fn find_user_by_id(db: &PgPool, id: &Uuid) -> Result<User, YuhuhError> {
    info!(id = ?id, "finding user by id");

    let user: User = sqlx::query_as(include_str!("queries/find_user_by_id.sql"))
        .bind(id)
        .fetch_one(db)
        .await
        .map_err(|e| {
            error!(error = ?e, user_id = ?id, "database error while finding user");

            match e {
                sqlx::Error::RowNotFound => YuhuhError::NotFound("user not found".to_string()),
                _ => YuhuhError::ContextError {
                    context: "failed to find user".to_string(),
                    error: Box::new(e),
                },
            }
        })?;

    info!(user_id = ?user.user_id, "user found successfully");
    Ok(user)
}

/// Queries the database for a user based on an associated discord ID.
/// If there is no found created discord user, then no user will be
/// returned. If present, the user will be returned with their
/// discord_user populated.
///
/// # Arguments
/// * `db` - PostgreSQL connection pool for database access
/// * `id` - discord id of the user to find
///
/// # Returns
/// * `Ok(User)` - The user record if found, with joined discord_user data
/// * `Err(YuhuhError::NotFound)` - If no user exists with the given discord ID
/// * `Err(YuhuhError::ContextError)` - If a database error occurs during the query
async fn find_user_by_discord_id(db: &PgPool, id: i64) -> Result<User, YuhuhError> {
    info!(id = ?id, "finding user by id");

    let user: User = sqlx::query_as(include_str!("queries/find_user_by_discord_id.sql"))
        .bind(id)
        .fetch_one(db)
        .await
        .map_err(|e| {
            error!(error = ?e, user_id = ?id, "database error while finding user");

            match e {
                sqlx::Error::RowNotFound => YuhuhError::NotFound("user not found".to_string()),
                _ => YuhuhError::ContextError {
                    context: "failed to find user".to_string(),
                    error: Box::new(e),
                },
            }
        })?;

    info!(user_id = ?user.user_id, "user found successfully");
    Ok(user)
}
