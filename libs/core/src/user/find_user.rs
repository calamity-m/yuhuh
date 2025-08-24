use axum::{Json, extract::Path, response::IntoResponse};
use serde::{Deserialize, Serialize};
use tracing::{debug, info};
use uuid::Uuid;

use crate::{error::CoreError, user::model::user::User};

// A simple data type we'll send and receive as JSON.
#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    content: String,
}

pub async fn find_user(Path(id): Path<Uuid>) -> Result<Json<User>, CoreError> {
    debug!("entered find_user - {}", id);

    Err(CoreError::NotImplemented)
}

// Handler for POST /messages
pub async fn test(Json(message): Json<Message>) -> impl IntoResponse {
    info!("Handling create_message request");
    Json(format!("New message: {}", message.content))
}
