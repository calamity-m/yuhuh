use axum::{Json, extract::Path};
use tracing::{debug, info, instrument};
use uuid::Uuid;

use crate::{error::CoreError, user::model::user::User};

#[instrument]
pub async fn find_user_id(Path(id): Path<Uuid>) -> Result<Json<User>, CoreError> {
    debug!("entered find_user_id - {}", id);

    Err(CoreError::NotImplemented)
}

#[instrument]
pub async fn find_user_uname(Path(username): Path<String>) -> Result<Json<User>, CoreError> {
    debug!("entered find_user_uname - {}", username);

    Err(CoreError::NotImplemented)
}
