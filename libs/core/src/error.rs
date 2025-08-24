use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Deserialize;
use thiserror::Error;

/// A common error type that can be used throughout the API.
///
/// Can be returned in a `Result` from an API handler function.
///
/// For convenience, this represents both API errors as well as internal recoverable errors,
/// and maps them to appropriate status codes along with at least a minimally useful error
/// message in a plain text body, or a JSON body in the case of `UnprocessableEntity`.
#[derive(Error, Debug)]
pub enum CoreError {
    #[error("Internal Server Error: {0}")]
    InternalServerError(String),

    #[error("Unauthorized")]
    Unauthorized,

    #[error("Resource not found")]
    NotFound(Option<String>),
}

#[derive(serde::Serialize, Deserialize, Debug)]
struct ErrorResponse<'a> {
    error: &'a str,
    reason: &'a str,
}

// Implement IntoResponse to convert AppError into an HTTP response
impl IntoResponse for CoreError {
    fn into_response(self) -> Response {
        match self {
            CoreError::InternalServerError(message) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: StatusCode::INTERNAL_SERVER_ERROR.as_str(),
                    reason: &message,
                }),
            )
                .into_response(),
            CoreError::Unauthorized => (
                StatusCode::UNAUTHORIZED,
                Json(ErrorResponse {
                    error: StatusCode::UNAUTHORIZED.as_str(),
                    reason: "unauthorized",
                }),
            )
                .into_response(),
            CoreError::NotFound(opt) => (
                StatusCode::NOT_FOUND,
                Json(ErrorResponse {
                    error: StatusCode::NOT_FOUND.as_str(),
                    reason: &opt.unwrap_or("resource not found".to_string()),
                }),
            )
                .into_response(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        Router,
        body::Body,
        http::{Request, StatusCode},
        routing::get,
    };
    use http_body_util::BodyExt;
    use tower::ServiceExt;

    #[tokio::test]
    async fn test_main_page() {
        fn app() -> Router {
            Router::new().route("/", get(handler))
        }

        async fn handler() -> Result<(), CoreError> {
            Err(CoreError::InternalServerError("bad stuff".to_string()))
        }
        let response = app()
            .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
        let body = response.into_body();
        let binding = body.collect().await.unwrap().to_bytes();
        let parsed: ErrorResponse = serde_json::from_slice(&binding).unwrap();

        assert_eq!(parsed.reason, "bad stuff");
    }
}
