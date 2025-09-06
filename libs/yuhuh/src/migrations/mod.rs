use sqlx::{PgPool, Postgres, migrate::MigrateDatabase, postgres::PgPoolOptions};
use tracing::{debug, error, info};

use crate::{config::Config, error::YuhuhError};

pub async fn run_migrations(config: &Config) -> Result<PgPool, YuhuhError> {
    debug!(url = ?config.database_url, "connecting to database");

    if !Postgres::database_exists(&config.database_url).await? {
        info!(
            "database {} not yet created; initializing it",
            &config.database_url
        );
        Postgres::create_database(&config.database_url).await?;
    }

    // We create a single connection pool for SQLx that's shared across the whole application.
    // This saves us from opening a new connection for every API call, which is wasteful.
    let db = PgPoolOptions::new()
        // The default connection limit for a Postgres server is 100 connections, minus 3 for superusers.
        // Since we're using the default superuser we don't have to worry about this too much,
        // although we should leave some connections available for manual access.
        //
        // If you're deploying your application with multiple replicas, then the total
        // across all replicas should not exceed the Postgres connection limit.
        .max_connections(50)
        .connect(&config.database_url)
        .await
        .map_err(|e| {
            error!(error = ?e, "encountered error connecting to database - {}", e);

            YuhuhError::InternalServerError(e.to_string())
        })?;

    run_migrations_with_db(db.clone()).await?;

    Ok(db)
}

pub async fn run_migrations_with_db(db: PgPool) -> Result<(), YuhuhError> {
    // Run user migrations
    sqlx::migrate!("src/migrations")
        .run(&db)
        .await
        .map_err(|e| {
            error!(error = ?e, "encountered error connecting to database - {}", e);

            YuhuhError::ContextError {
                context: "failed migrations".to_string(),
                error: Box::new(e),
            }
        })?;

    Ok(())
}
