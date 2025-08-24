use std::sync::Arc;

use anyhow::{Context, Ok, Result};
use axum::extract::FromRef;
use axum::{Router, http::HeaderName};
use sqlx::postgres::PgPoolOptions;
use tower_http::trace::{DefaultOnFailure, DefaultOnResponse};
use tracing::info;
use tracing::{Level, Span};

use crate::config::Config;
use crate::user::UserState;
use crate::user::find_user::service::FindUserService;
use crate::user::model::user::User;

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

        #[clap(long, env)]
        pub database_url: String,
    }
}

#[derive(Clone)]
pub struct AppState {
    config: Arc<Config>,
    user_state: Arc<UserState>,
}

impl FromRef<AppState> for Arc<Config> {
    fn from_ref(input: &AppState) -> Self {
        input.config.clone()
    }
}

impl FromRef<AppState> for Arc<UserState> {
    fn from_ref(input: &AppState) -> Self {
        input.user_state.clone()
    }
}

pub async fn serve(config: &config::Config) -> Result<()> {
    info!("serving");

    // Grab the current span here as our "global span",
    // so that we can just add to it with our middlewares
    // and whatever other layers need to deal with
    // tracing
    let global_span = Arc::new(Span::current());

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

    let app_state = AppState {
        config: Arc::new(config.clone()),
        user_state: Arc::new(UserState::new(db)),
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
