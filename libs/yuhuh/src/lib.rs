pub mod api;
pub mod config;
pub mod error;
pub mod health;
pub mod migrations;
pub mod state;
pub mod user;

use anyhow::{Context, Ok, Result};
use tracing::info;

pub async fn main(config: &config::Config) -> Result<()> {
    let global_span = log::init(
        &log::CommonFields {
            system: "yuhuh",
            version: "0.0.1",
        },
        &config.log,
    )
    .expect("logging initialisation failed");

    let _guard = global_span.enter();

    info!("initialized logging");

    let db = migrations::run_migrations(config).await?;

    info!("db connection and setup successful");

    // Spin up API
    api::serve(config, db)
        .await
        .with_context(|| "failed to run yuhuh")?;

    Ok(())
}
