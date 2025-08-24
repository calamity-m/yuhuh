use std::sync::Arc;

use anyhow::{Context, Ok, Result};
use axum::extract::FromRef;
use axum::{Router, http::HeaderName, routing::get};
use tower_http::trace::{DefaultOnFailure, DefaultOnResponse};
use tracing::info;
use tracing::{Level, Span};

use crate::config::Config;

pub mod error;

pub mod middleware {
    pub mod requestid;
}

pub mod health;

pub mod user;

pub mod config {
    #[derive(clap::Parser, Debug, Clone)]
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

#[derive(Clone)]
pub struct AppState {
    config: Arc<Config>,
}

impl FromRef<AppState> for Arc<Config> {
    fn from_ref(input: &AppState) -> Self {
        input.config.clone()
    }
}

pub async fn serve(config: &config::Config) -> Result<()> {
    info!("serving");

    // Grab the current span here as our "global span",
    // so that we can just add to it with our middlewares
    // and whatever other layers need to deal with
    // tracing
    let global_span = Arc::new(Span::current());

    let app_state = AppState {
        config: Arc::new(config.clone()),
    };

    let app = Router::new()
        .merge(health::health_router())
        .merge(user::user_router())
        .with_state(app_state.clone())
        .fallback(|| async {
            // Return the core not found error with a nice message for our caller
            error::CoreError::NotFound(Some("no matching route found".to_string()))
        })
        .layer(
            tower_http::trace::TraceLayer::new_for_http()
                .on_failure(DefaultOnFailure::new().level(Level::ERROR))
                .on_response(
                    DefaultOnResponse::new()
                        .include_headers(true)
                        .level(Level::INFO),
                ),
        )
        .layer(tower_http::request_id::PropagateRequestIdLayer::new(
            HeaderName::from_static(middleware::requestid::REQUEST_ID_HEADER),
        ))
        .layer(middleware::requestid::LogRequestIdLayer::new(global_span))
        .layer(tower_http::request_id::SetRequestIdLayer::new(
            HeaderName::from_static(middleware::requestid::REQUEST_ID_HEADER),
            tower_http::request_id::MakeRequestUuid,
        ));

    let listener = tokio::net::TcpListener::bind(format!("127.0.0.1:{}", config.port))
        .await
        .with_context(|| format!("failed to serve on port {}", config.port))?;

    info!("serving on port {}", config.port);

    axum::serve(listener, app).await.unwrap();

    Ok(())
}
