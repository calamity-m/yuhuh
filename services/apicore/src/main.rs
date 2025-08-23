use anyhow::{Context, Ok, Result};

use clap::Parser;
use tracing::{Level, info};

#[tokio::main]
async fn main() -> Result<()> {
    // This returns an error if the `.env` file doesn't exist, but that's not what we want
    // since we're not going to use a `.env` file if we deploy this application.
    dotenvy::dotenv().ok();

    let config = core::config::Config::parse();

    tracing_subscriber::fmt()
        // filter spans/events with level TRACE or higher.
        .with_max_level(Level::TRACE)
        // build but do not install the subscriber.
        .init();

    info!("Starting API Core");

    // Spin up API
    core::serve(&config)
        .await
        .with_context(|| "failed to run apicore")?;

    Ok(())
}
