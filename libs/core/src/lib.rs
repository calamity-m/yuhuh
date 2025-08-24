use std::sync::Arc;

use anyhow::{Context, Ok, Result};
use axum::{Router, http::HeaderName, response::Html, routing::get};
use tower_http::{
    request_id::{MakeRequestUuid, PropagateRequestIdLayer, SetRequestIdLayer},
    trace::TraceLayer,
};
use tracing::Span;
use tracing::info;

use crate::{config::Config, error::CoreError, middleware::requestid::LogRequestIdLayer};

pub mod config;

pub mod error;

pub mod middleware {
    pub mod requestid;
}

pub mod dummy;

const REQUEST_ID_HEADER: &str = "x-request-id";

pub async fn serve(config: &Config) -> Result<()> {
    let x_request_id = HeaderName::from_static(REQUEST_ID_HEADER);

    info!("serving");

    // Grab the current span here as our "global span",
    // so that we can just add to it with our middlewares
    // and whatever other layers need to deal with
    // tracing
    let global_span = Arc::new(Span::current());

    let app = Router::new()
        .route("/", get(handler))
        .route(
            "/dummy",
            get(dummy::list_messages).post(dummy::create_message),
        )
        .fallback(|| async {
            // Return the core not found error with a nice message for our caller
            CoreError::NotFound(Some("no matching route found".to_string()))
        })
        .layer(TraceLayer::new_for_http())
        .layer(LogRequestIdLayer::new(global_span))
        .layer(SetRequestIdLayer::new(
            x_request_id.clone(),
            MakeRequestUuid,
        ))
        .layer(PropagateRequestIdLayer::new(x_request_id));

    let listener = tokio::net::TcpListener::bind(format!("127.0.0.1:{}", config.port))
        .await
        .with_context(|| format!("failed to serve on port {}", config.port))?;

    info!("serving on port {}", config.port);

    axum::serve(listener, app).await.unwrap();

    Ok(())
}

async fn handler() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}
