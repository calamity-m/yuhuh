use std::sync::Arc;

use anyhow::{Context, Ok, Result};
use axum::{Router, http::HeaderName};
use sqlx::PgPool;
use tower_http::trace::{DefaultOnFailure, DefaultOnResponse};
use tracing::info;
use tracing::{Level, Span};

use middleware::*;
use utoipa::OpenApi;
use utoipa_axum::router::OpenApiRouter;
use utoipa_scalar::{Scalar, Servable};

use crate::config::Config;
use crate::error::*;
use crate::health::*;
use crate::state::create_app_state;
use crate::user::router::user_router;

#[derive(OpenApi)]
#[openapi(
    tags(
        (name = "yuhuh", description = "Todo items management API")
    ),
    nest(
        (path="/api/v1/", api = crate::user::router::UserApi),
    )
)]
struct ApiDoc;

pub fn new_app(config: &Config, db: PgPool, global_span: Arc<Span>) -> Router {
    let app_state = create_app_state(config, db);

    Router::new()
        .merge(health_router())
        .merge(user_router())
        //.merge(user::user_router())
        .with_state(app_state.clone())
        .fallback(|| async {
            // Return the core not found error with a nice message for our caller
            YuhuhError::NotFound("no matching route found".to_string())
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
        ))
}

pub async fn serve(config: &Config, db: PgPool) -> Result<()> {
    info!("serving");

    // Grab the current span here as our "global span",
    // so that we can just add to it with our middlewares
    // and whatever other layers need to deal with
    // tracing
    let global_span = Arc::new(Span::current());

    let app: Router = new_app(config, db, global_span);

    let (app_openapi, api) = OpenApiRouter::with_openapi(ApiDoc::openapi())
        .nest("/api/v1", app.into())
        .split_for_parts();

    let app_openapi = app_openapi.merge(Scalar::with_url("/scalar", api));

    let listener = tokio::net::TcpListener::bind(format!("127.0.0.1:{}", config.port))
        .await
        .with_context(|| format!("failed to serve on port {}", config.port))?;

    info!("serving on port {}", config.port);

    axum::serve(listener, app_openapi).await.unwrap();

    Ok(())
}
