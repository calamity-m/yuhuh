use axum::Json;
use axum::response::IntoResponse;
use serde::{Deserialize, Serialize};
use tracing::info;

// A simple data type we'll send and receive as JSON.
#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    content: String,
}

// Handler for GET /messages
pub async fn list_messages() -> impl IntoResponse {
    info!("Handling list_messages request");
    Json(vec!["Hello from the server!".to_string()])
}

// Handler for POST /messages
pub async fn create_message(Json(message): Json<Message>) -> impl IntoResponse {
    info!("Handling create_message request");
    Json(format!("New message: {}", message.content))
}
