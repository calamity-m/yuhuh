//! Find user repository module
//!
//! This module provides functionality for finding users from the database.

use async_trait::async_trait;
use sqlx::PgPool;
use tracing::{debug, error, info};
use uuid::Uuid;

use crate::{error::YuhuhError, user::model::User};

#[async_trait]
pub trait FindUserRepository: std::fmt::Debug + Send + Sync + 'static {
    async fn find_user_by_id(&self, id: &Uuid) -> Result<Option<User>, YuhuhError>;
    async fn find_user_by_discord_id(&self, id: i64) -> Result<Option<User>, YuhuhError>;
}

#[derive(Debug)]
pub struct FindUserRepositoryImpl {
    pub db: PgPool,
}

#[async_trait]
impl FindUserRepository for FindUserRepositoryImpl {
    async fn find_user_by_id(&self, id: &Uuid) -> Result<Option<User>, YuhuhError> {
        info!(id = ?id, "finding user by id");

        let user: Option<User> = sqlx::query_as(include_str!("queries/find_user_by_id.sql"))
            .bind(id)
            .fetch_optional(&self.db)
            .await
            .map_err(|e| {
                error!(error = ?e, user_id = ?id, "database error while finding user");

                YuhuhError::ContextError {
                    context: "failed to find user".to_string(),
                    error: Box::new(e),
                }
            })?;

        debug!(user_id = ?user, "reponse from query");
        Ok(user)
    }

    async fn find_user_by_discord_id(&self, id: i64) -> Result<Option<User>, YuhuhError> {
        info!(id = ?id, "finding user by id");

        let user: Option<User> =
            sqlx::query_as(include_str!("queries/find_user_by_discord_id.sql"))
                .bind(id)
                .fetch_optional(&self.db)
                .await
                .map_err(|e| {
                    error!(error = ?e, user_id = ?id, "database error while finding user");

                    YuhuhError::ContextError {
                        context: "failed to find user".to_string(),
                        error: Box::new(e),
                    }
                })?;

        debug!(user_id = ?user, "reponse from query");
        Ok(user)
    }
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct DummyFindUserRepository {}

#[async_trait]
impl FindUserRepository for DummyFindUserRepository {
    async fn find_user_by_id(&self, _id: &Uuid) -> Result<Option<User>, YuhuhError> {
        panic!(
            "DummyFindUserRepository::find_user_by_id called - this should be unreachable in tests"
        );
    }

    async fn find_user_by_discord_id(&self, _id: i64) -> Result<Option<User>, YuhuhError> {
        panic!(
            "DummyFindUserRepository::find_user_by_discord_id called - this should be unreachable in tests"
        );
    }
}
