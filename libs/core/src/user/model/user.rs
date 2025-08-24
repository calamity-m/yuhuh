use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    id: Uuid,
    username: String,
    created_time: DateTime<Utc>,
    last_modified: DateTime<Utc>,
}
