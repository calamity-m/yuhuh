//! user creation HTTP handler
//!
//! This module provides HTTP endpoints for creating users. It handles
//! request validation, duplicate checking, and user creation through the
//! repository layer.

use std::sync::Arc;

use axum::{Json, extract::State, http::StatusCode};
use serde::{Deserialize, Serialize};
use tracing::{info, instrument, warn};
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

use crate::{error::YuhuhError, user::state::UserState};

// =============================================================================
// Request/Response Types
// =============================================================================

/// Request payload for creating a Discord user via HTTP API.
///
/// This struct represents the JSON payload expected by the POST endpoint
/// for creating Discord users. It includes validation constraints to ensure
/// data integrity before processing.
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreateDiscordUserRequest {
    /// Discord user ID (snowflake as i64)
    pub discord_id: i64,
    /// Discord username
    pub discord_username: String,
    /// Optional user personalization settings
    pub personalisation: Option<String>,
    /// Optional contact name
    pub contact_name: Option<String>,
    /// Optional contact email (must be valid email format if provided)
    #[validate(email)]
    pub contact_email: Option<String>,
    /// Optional timezone preference
    pub timezone: Option<String>,
}

/// Response payload for successful Discord user creation.
///
/// Contains the UUID of the newly created user that can be used
/// for subsequent API calls or client-side operations.
#[derive(Debug, Serialize, ToSchema)]
pub struct CreateDiscordUserResponse {
    /// The UUID of the newly created user
    pub user_id: Uuid,
}

// =============================================================================
// HTTP Handlers
// =============================================================================

/// Creates a new Discord user with associated user data.
#[utoipa::path(
        post,
        path = "users/create/discord",
        tag = "create user",
        responses(
            (status = 201, description = "Discord user created successfully", body = CreateDiscordUserResponse),
            (status = 400, description = "User exists")
        )
    )]
#[axum::debug_handler]
#[instrument]
pub async fn post_create_discord_user(
    State(user_state): State<Arc<UserState>>,
    Json(request): Json<CreateDiscordUserRequest>,
) -> Result<(StatusCode, Json<CreateDiscordUserResponse>), YuhuhError> {
    // Check if a user with this Discord ID already exists
    if let Some(user) = user_state
        .find_user_repo
        .find_user_by_discord_id(request.discord_id)
        .await?
    {
        warn!(user = ?user, "existing user found");

        return Err(YuhuhError::BadRequest(format!(
            "User with Discord ID {} already exists",
            request.discord_id,
        )));
    }

    // Create the user through the repository layer
    let created_user_id = user_state
        .create_user_repo
        .create_discord_user(super::CreateDBDiscordUserRequest {
            discord_id: request.discord_id,
            discord_username: request.discord_username,
            personalisation: request.personalisation,
            contact_name: request.contact_name,
            contact_email: request.contact_email,
            timezone: request.timezone,
        })
        .await?;

    info!(
        user_id = %created_user_id,
        discord_id = request.discord_id,
        "Successfully created Discord user"
    );

    Ok((
        StatusCode::CREATED,
        Json(CreateDiscordUserResponse {
            user_id: created_user_id,
        }),
    ))
}
