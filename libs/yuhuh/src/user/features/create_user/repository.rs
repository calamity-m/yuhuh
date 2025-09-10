//! repository

use async_trait::async_trait;
use sqlx::{PgPool, Postgres, prelude::FromRow};
use tracing::{error, info};
use uuid::Uuid;

use crate::error::YuhuhError;

#[async_trait]
pub trait CreateUserRepository: std::fmt::Debug + Send + Sync + 'static {
    /// Queries the database for a user based on their ID. Returns a user joined
    /// with their discord_user if present, wrapped in an Ok, None otherwise.
    ///
    /// # Arguments
    /// * `db` - PostgreSQL connection pool for database access
    /// * `id` - UUID of the user to find
    ///
    /// # Returns
    /// * `Ok(User)` - The user record if found, with joined discord_user data if available
    /// * `None` - If no user exists with the given ID
    /// * `Err(YuhuhError::ContextError)` - If a database error occurs during the query
    async fn create_discord_user(&self, id: CreateDiscordUserRequest) -> Result<Uuid, YuhuhError>;
}

#[derive(Debug)]
pub struct CreateUserRepositoryImpl {
    pub db: PgPool,
}

#[derive(Debug)]
pub struct CreateDiscordUserRequest {
    pub discord_id: i64,
    pub discord_username: String,
    pub personalisation: Option<String>,
    pub contact_name: Option<String>,
    pub contact_email: Option<String>,
    pub timezone: Option<String>,
}

#[derive(FromRow)]
struct CreateUserRow {
    pub user_id: Uuid,
}

#[async_trait]
impl CreateUserRepository for CreateUserRepositoryImpl {
    async fn create_discord_user(
        &self,
        request: CreateDiscordUserRequest,
    ) -> Result<Uuid, YuhuhError> {
        // The user doesn't exist, so grab a transaction lock
        let mut transaction = self.db.begin().await?;

        let id = sqlx::query_as!(
            CreateUserRow,
            r#"
            INSERT INTO users (
                personalisation,
                contact_email,
                contact_name,
                timezone
            ) VALUES (
                $1, 
                $2, 
                $3, 
                $4
            ) RETURNING user_id
            "#,
            request.personalisation,
            request.contact_email,
            request.contact_name,
            request.timezone
        )
        .fetch_one(&mut *transaction)
        .await?
        .user_id;

        sqlx::query!(
            r#"
        INSERT INTO discord_users (
            discord_id,
            username,
            user_id
        ) VALUES (
            $1,
            $2,
            $3
        )
        "#,
            request.discord_id,
            request.discord_username,
            id
        )
        .execute(&mut *transaction)
        .await?;

        transaction.commit().await?;

        Ok(id)
    }
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct DummyCreateUserRepository {}

#[async_trait]
impl CreateUserRepository for DummyCreateUserRepository {
    async fn create_discord_user(
        &self,
        _request: CreateDiscordUserRequest,
    ) -> Result<Uuid, YuhuhError> {
        todo!()
    }
}
