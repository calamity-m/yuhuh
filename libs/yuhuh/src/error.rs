use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Deserialize;
use thiserror::Error;

#[derive(Error, Debug)]
#[error("Failed conversion: {message}")]
pub struct ConversionError {
    message: String,
}

impl ConversionError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

#[derive(Error, Debug)]
#[error("Invalid rating: {message}")]
pub struct RatingError {
    message: String,
}

impl RatingError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

/// A common error type that can be used throughout the API.
///
/// Can be returned in a `Result` from an API handler function.
///
/// For convenience, this represents both API errors as well as internal recoverable errors,
/// and maps them to appropriate status codes along with at least a minimally useful error
/// message in a plain text body, or a JSON body in the case of `UnprocessableEntity`.
#[derive(Error, Debug)]
pub enum YuhuhError {
    #[error("Internal Server Error: {0}")]
    InternalServerError(String),

    #[error("Unauthorized")]
    Unauthorized,

    #[error("Resource not found")]
    NotFound(String),

    #[error(transparent)]
    ValidationError(#[from] validator::ValidationErrors),

    #[error("Not yet implemented")]
    NotImplemented,

    #[error("Bad request")]
    BadRequest(String),

    #[error("Database error")]
    DatabaseError(#[from] sqlx::Error),

    #[error("{0}")]
    Conflict(String),

    #[error("Context error")]
    ContextError {
        context: String,
        error: Box<dyn std::error::Error + 'static + Send + Sync>,
    },

    #[error(transparent)]
    RatingError(#[from] RatingError),

    #[error(transparent)]
    ConversionError(#[from] ConversionError),
}

#[derive(serde::Serialize, Deserialize, Debug)]
struct ErrorResponse<'a> {
    error: &'a str,
    reason: &'a str,
}

// Implement IntoResponse to convert AppError into an HTTP response
impl IntoResponse for YuhuhError {
    fn into_response(self) -> Response {
        self.response()
    }
}

impl YuhuhError {
    fn response(&self) -> Response {
        (
            self.status_code(),
            Json(ErrorResponse {
                error: self.status_code().as_str(),
                reason: &self.message(),
            }),
        )
            .into_response()
    }

    fn message(&self) -> String {
        match self {
            YuhuhError::InternalServerError(err)
            | YuhuhError::BadRequest(err)
            | YuhuhError::NotFound(err)
            | YuhuhError::Conflict(err) => {
                tracing::error!(error=?err, "encountered error - message: {}", err);

                err.to_string()
            }
            YuhuhError::Unauthorized => {
                tracing::error!("encountered unauthorized error");

                "unauthorized".to_string()
            }
            YuhuhError::ValidationError(validation_errors) => {
                tracing::error!(error=?validation_errors, "encountered validation errors");

                let message =
                    format!("Input validation error: [{validation_errors}]").replace('\n', ", ");

                message.to_owned()
            }
            YuhuhError::NotImplemented => {
                tracing::error!("encountered not implemented error");

                "not implemented".to_string()
            }
            YuhuhError::DatabaseError(error) => {
                tracing::error!(error=?error, "encountered database error");

                "internal error occured".to_string()
            }
            YuhuhError::ContextError { context, error } => {
                tracing::error!(error=?error, "encountered context error");

                context.clone()
            }
            YuhuhError::RatingError(error) => {
                tracing::error!(error=?error, "encountered rating error");

                error.message.to_owned()
            }
            YuhuhError::ConversionError(error) => {
                tracing::error!(error=?error, "encountered conversion error");

                error.message.to_owned()
            }
        }
    }

    fn status_code(&self) -> StatusCode {
        match self {
            YuhuhError::InternalServerError(_)
            | YuhuhError::DatabaseError(_)
            | YuhuhError::NotImplemented
            | YuhuhError::ConversionError(_)
            | YuhuhError::ContextError { .. } => StatusCode::INTERNAL_SERVER_ERROR,
            YuhuhError::Unauthorized => StatusCode::UNAUTHORIZED,
            YuhuhError::NotFound(_) => StatusCode::NOT_FOUND,
            YuhuhError::Conflict(_)
            | YuhuhError::BadRequest(_)
            | YuhuhError::RatingError(_)
            | YuhuhError::ValidationError(_) => StatusCode::BAD_REQUEST,
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

        async fn handler() -> Result<(), YuhuhError> {
            Err(YuhuhError::InternalServerError("bad stuff".to_string()))
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
