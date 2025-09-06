use std::sync::Arc;

use anyhow::{Context, Ok, Result};
use axum::{Router, http::HeaderName};
use sqlx::PgPool;
use tower_http::trace::{DefaultOnFailure, DefaultOnResponse};
use tracing::info;
use tracing::{Level, Span};

use middleware::*;

use crate::config::Config;
use crate::error::*;
use crate::health::*;
use crate::state::{AppState, create_app_state};
use crate::user::router::user_router;

pub async fn serve(config: &Config, db: PgPool) -> Result<()> {
    info!("serving");

    // Grab the current span here as our "global span",
    // so that we can just add to it with our middlewares
    // and whatever other layers need to deal with
    // tracing
    let global_span = Arc::new(Span::current());

    let app_state = create_app_state(config, db);

    let app = Router::new()
        .merge(health_router())
        .merge(user_router())
        //.merge(user::user_router())
        .with_state(app_state.clone())
        .fallback(|| async {
            // Return the core not found error with a nice message for our caller
            YuhuhError::NotFound(Some("no matching route found".to_string()))
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
            HeaderName::from_static(requestid::REQUEST_ID_HEADER),
        ))
        .layer(requestid::LogRequestIdLayer::new(global_span))
        .layer(tower_http::request_id::SetRequestIdLayer::new(
            HeaderName::from_static(requestid::REQUEST_ID_HEADER),
            tower_http::request_id::MakeRequestUuid,
        ));

    let listener = tokio::net::TcpListener::bind(format!("127.0.0.1:{}", config.port))
        .await
        .with_context(|| format!("failed to serve on port {}", config.port))?;

    info!("serving on port {}", config.port);

    axum::serve(listener, app).await.unwrap();

    Ok(())
}
