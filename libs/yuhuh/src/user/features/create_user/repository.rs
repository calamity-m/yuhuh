//! User creation repository module
//!
//! This module provides functionality for creating users in the database.

use async_trait::async_trait;
use sqlx::{PgPool, prelude::FromRow};
use uuid::Uuid;

use crate::error::YuhuhError;

// =============================================================================
// Public Types and Structs
// =============================================================================

/// Request struct for creating a Discord user with associated user data.
///
/// Contains all the necessary information to create both a user record and
/// a linked Discord user record in the database.
#[derive(Debug)]
pub struct CreateDiscordUserRequest {
    /// Discord user ID (snowflake as i64)
    pub discord_id: i64,
    /// Discord username
    pub discord_username: String,
    /// Optional user personalization settings
    pub personalisation: Option<String>,
    /// Optional contact name
    pub contact_name: Option<String>,
    /// Optional contact email
    pub contact_email: Option<String>,
    /// Optional timezone preference
    pub timezone: Option<String>,
}

// =============================================================================
// Private Types
// =============================================================================

/// Internal struct for mapping database query results when creating users.
#[derive(FromRow)]
struct CreateUserRow {
    /// The UUID of the newly created user
    pub user_id: Uuid,
}

// =============================================================================
// Traits
// =============================================================================

/// Trait for repositories that handle user creation operations.
///
/// This trait defines the interface for creating Discord users with associated
/// user data. Implementations should handle both user and discord_user table
/// insertions within a transaction to maintain data consistency.
#[async_trait]
pub trait CreateUserRepository: std::fmt::Debug + Send + Sync + 'static {
    /// Creates a new user with associated Discord account information.
    ///
    /// This method creates both a user record and a linked discord_user record
    /// within a database transaction to ensure data consistency. The user is
    /// created first, and then the Discord user record is linked to it.
    ///
    /// # Arguments
    /// * `request` - The user creation request containing all necessary data
    ///
    /// # Returns
    /// * `Ok(Uuid)` - The UUID of the newly created user
    /// * `Err(YuhuhError)` - If a database error occurs during the transaction
    ///
    /// # Errors
    /// This method will return an error if:
    /// - The database transaction fails to begin
    /// - The user insertion fails (e.g., constraint violations)
    /// - The discord_user insertion fails
    /// - The transaction fails to commit
    async fn create_discord_user(
        &self,
        request: CreateDiscordUserRequest,
    ) -> Result<Uuid, YuhuhError>;
}

// =============================================================================
// Production Implementation
// =============================================================================

/// Production implementation of the CreateUserRepository trait.
///
/// This implementation uses a PostgreSQL connection pool to perform database
/// operations. It handles user creation with proper transaction management
/// to ensure data consistency.
#[derive(Debug)]
pub struct CreateUserRepositoryImpl {
    /// PostgreSQL connection pool for database access
    pub db: PgPool,
}

#[async_trait]
impl CreateUserRepository for CreateUserRepositoryImpl {
    async fn create_discord_user(
        &self,
        request: CreateDiscordUserRequest,
    ) -> Result<Uuid, YuhuhError> {
        // Begin a database transaction to ensure atomicity
        let mut transaction = self.db.begin().await?;

        // First, create the user record and get the generated UUID
        let user_id = sqlx::query_as!(
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

        // Then, create the linked Discord user record
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
            user_id
        )
        .execute(&mut *transaction)
        .await?;

        // Commit the transaction to persist all changes
        transaction.commit().await?;

        Ok(user_id)
    }
}

// =============================================================================
// Test/Mock Implementation
// =============================================================================

/// Dummy implementation of CreateUserRepository for testing purposes.
///
/// This implementation provides a placeholder for testing scenarios where
/// actual database operations are not needed. The methods are marked with
/// `todo!()` and are intended to auto fail. Any testing beyond this point
/// should revert to integration testing.
#[derive(Debug)]
#[allow(dead_code)]
pub struct DummyCreateUserRepository {}

#[async_trait]
impl CreateUserRepository for DummyCreateUserRepository {
    async fn create_discord_user(
        &self,
        _request: CreateDiscordUserRequest,
    ) -> Result<Uuid, YuhuhError> {
        panic!(
            "DummyCreateUserRepository::create_discord_user called - this should be unreachable in tests"
        );
    }
}
