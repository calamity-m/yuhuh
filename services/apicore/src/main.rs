use anyhow::{Context, Ok, Result};

use clap::Parser;
use sqlx::postgres::PgPoolOptions;
use tracing::{debug, info};

#[tokio::main]
async fn main() -> Result<()> {
    // This returns an error if the `.env` file doesn't exist, but that's not what we want
    // since we're not going to use a `.env` file if we deploy this application.
    dotenvy::dotenv().ok();

    let config = yuhuh::config::Config::parse();

    let global_span = log::init(
        &log::CommonFields {
            system: "core",
            version: "0.0.1",
        },
        &config.log,
    )
    .expect("logging initialisation failed");

    let _guard = global_span.enter();

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
        .context("could not connect to database_url")?;

    info!("Starting API Core");

    debug!("hello there");

    info!("config: {:?}", config);

    // Spin up API
    yuhuh::handler::serve(&config, db)
        .await
        .with_context(|| "failed to run apicore")?;

    Ok(())
}
