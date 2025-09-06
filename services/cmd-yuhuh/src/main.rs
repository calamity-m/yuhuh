use anyhow::{Ok, Result};

use clap::Parser;

#[tokio::main]
async fn main() -> Result<()> {
    // This returns an error if the `.env` file doesn't exist, but that's not what we want
    // since we're not going to use a `.env` file if we deploy this application.
    dotenvy::dotenv().ok();

    let config = yuhuh::config::Config::parse();

    yuhuh::main(&config).await?;

    Ok(())
}
