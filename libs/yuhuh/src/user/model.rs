use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    id: Uuid,
    personalisation: Option<String>,
    contact_email: Option<String>,
    contact_name: Option<String>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    timezone: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiscordUser {
    id: i64,
    username: String,
    user_id: Uuid,
}
