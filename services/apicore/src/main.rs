use anyhow::{Context, Ok, Result};

use clap::Parser;
use tracing::{debug, info};

#[tokio::main]
async fn main() -> Result<()> {
    // This returns an error if the `.env` file doesn't exist, but that's not what we want
    // since we're not going to use a `.env` file if we deploy this application.
    dotenvy::dotenv().ok();

    let config = core::config::Config::parse();

    let global_span = log::init(
        &log::CommonFields {
            system: "core",
            version: "0.0.1",
        },
        &config.log,
    )
    .expect("logging initialisation failed");

    let _guard = global_span.enter();

    info!("Starting API Core");

    debug!("hello there");

    info!("config: {:?}", config);

    // Spin up API
    core::serve(&config)
        .await
        .with_context(|| "failed to run apicore")?;

    Ok(())
}
