use std::sync::Arc;

use anyhow::{Context, Ok, Result};
use axum::Json;
use axum::response::IntoResponse;
use axum::{Router, http::HeaderName, response::Html, routing::get};
use serde::Serialize;
use tracing::Span;
use tracing::info;

pub mod error;

pub mod middleware {
    pub mod requestid;
}

pub mod dummy;

pub mod config {
    #[derive(clap::Parser, Debug)]
    pub struct Config {
        /// Port to serve core on.
        ///
        /// Defaults to 3000
        #[clap(long, env)]
        #[arg(default_value_t = 3000)]
        pub port: u16,

        #[clap(flatten)]
        pub log: log::LoggingConfig,
    }
}

pub async fn serve(config: &config::Config) -> Result<()> {
    info!("serving");

    // Grab the current span here as our "global span",
    // so that we can just add to it with our middlewares
    // and whatever other layers need to deal with
    // tracing
    let global_span = Arc::new(Span::current());

    let app = Router::new()
        .route("/", get(root))
        .route(
            "/dummy",
            get(dummy::list_messages).post(dummy::create_message),
        )
        .fallback(|| async {
            // Return the core not found error with a nice message for our caller
            error::CoreError::NotFound(Some("no matching route found".to_string()))
        })
        .layer(tower_http::trace::TraceLayer::new_for_http())
        .layer(middleware::requestid::LogRequestIdLayer::new(global_span))
        .layer(tower_http::request_id::SetRequestIdLayer::new(
            HeaderName::from_static(middleware::requestid::REQUEST_ID_HEADER),
            tower_http::request_id::MakeRequestUuid,
        ))
        .layer(tower_http::request_id::PropagateRequestIdLayer::new(
            HeaderName::from_static(middleware::requestid::REQUEST_ID_HEADER),
        ));

    let listener = tokio::net::TcpListener::bind(format!("127.0.0.1:{}", config.port))
        .await
        .with_context(|| format!("failed to serve on port {}", config.port))?;

    info!("serving on port {}", config.port);

    axum::serve(listener, app).await.unwrap();

    Ok(())
}

async fn root() -> impl IntoResponse {
    #[derive(Serialize)]
    struct Message {
        system: String,
    }
    Json(Message {
        system: "core".to_string(),
    })
}
