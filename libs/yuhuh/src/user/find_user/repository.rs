use async_trait::async_trait;
use sqlx::{PgPool, Postgres};
use tracing::{error, info};

use crate::{error::YuhuhError, user::model::User};

#[async_trait]
pub trait FindUserRepository: std::fmt::Debug + Send + Sync + 'static {
    async fn find_user(&self, id: &uuid::Uuid) -> Result<User, YuhuhError>;
}

#[derive(Debug)]
pub struct FindUserRepositoryImpl {
    pub db: PgPool,
}

#[async_trait]
impl FindUserRepository for FindUserRepositoryImpl {
    async fn find_user(&self, id: &uuid::Uuid) -> Result<User, YuhuhError> {
        info!(id = ?id, "called find user with id");

        // Search for the user via ID
        let user: User = sqlx::query_as(
            r#"
            SELECT 
                u.user_id,
                u.personalisation,
                u.contact_email,
                u.contact_name,
                u.created_at,
                u.updated_at,
                u.timezone,
                to_json(du.*) AS discord_user
            FROM users u
            LEFT JOIN discord_users du ON u.user_id = du.user_id
            WHERE u.user_id = $1"#,
        )
        .bind(id)
        .fetch_one(&self.db)
        .await
        .map_err(|e| {
            error!(error=?e, "encountered database error");

            match e {
                // Row not found may be common and not due to a database failure
                sqlx::Error::RowNotFound => YuhuhError::NotFound("user not found".to_string()),
                _ => YuhuhError::ContextError {
                    context: "failed to find user".to_string(),
                    error: Box::new(e),
                },
            }
        })?;

        info!(user=?user, "found user");

        Ok(user)
    }
}

#[derive(Debug)]
pub struct DummyFindUserRepository {}

#[async_trait]
impl FindUserRepository for DummyFindUserRepository {
    async fn find_user(&self, id: &uuid::Uuid) -> Result<User, YuhuhError> {
        info!(id = ?id, "called with id");
        todo!()
    }
}
