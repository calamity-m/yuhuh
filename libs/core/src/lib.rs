use anyhow::{Context, Ok, Result};
use axum::{Router, response::Html, routing::get};
use tracing::info;

use crate::config::Config;

pub mod config;

pub mod error;

pub async fn serve(config: &Config) -> Result<()> {
    let app = Router::new().route("/", get(handler));

    let listener = tokio::net::TcpListener::bind(format!("127.0.0.1:{}", config.port))
        .await
        .with_context(|| format!("failed to serve on port {}", config.port))?;

    info!(system = "apicore", "serving on port {}", config.port);

    axum::serve(listener, app).await.unwrap();

    Ok(())
}

async fn handler() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}
