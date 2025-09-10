use std::sync::Arc;

use axum::{Json, extract::State};
use serde::{Deserialize, Serialize};
use tracing::{info, instrument};
use uuid::Uuid;
use validator::Validate;

use crate::{
    error::YuhuhError,
    user::{
        features::{create_user, find_user},
        state::UserState,
    },
};

#[derive(Debug, Deserialize, Validate)]
pub struct CreateDiscordUserRequest {
    pub discord_id: i64,
    pub discord_username: String,
    pub personalisation: Option<String>,
    pub contact_name: Option<String>,
    #[validate(email)]
    pub contact_email: Option<String>,
    pub timezone: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct CreateDiscordUserResponse {
    pub user_id: Uuid,
}

#[axum::debug_handler]
#[instrument]
pub async fn post_create_discord_user(
    State(user_state): State<Arc<UserState>>,
    Json(request): Json<CreateDiscordUserRequest>,
) -> Result<Json<CreateDiscordUserResponse>, YuhuhError> {
    //Check if some user for this id already exists
    if user_state
        .find_user_repo
        .find_user_by_discord_id(request.discord_id)
        .await
        .is_ok()
    {
        return Err(YuhuhError::BadRequest(format!(
            "user for discord_id {} already exists",
            request.discord_id,
        )));
    }

    // create the user
    let created_user_id = user_state
        .create_user_repo
        .create_discord_user(create_user::repository::CreateDiscordUserRequest {
            discord_id: request.discord_id,
            discord_username: request.discord_username,
            personalisation: request.personalisation,
            contact_name: request.contact_name,
            contact_email: request.contact_email,
            timezone: request.timezone,
        })
        .await?;

    info!("created user with id - {}", created_user_id);

    Ok(Json(CreateDiscordUserResponse {
        user_id: created_user_id,
    }))
}
