use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    pub user_id: Uuid,
    pub personalisation: Option<String>,
    pub contact_email: Option<String>,
    pub contact_name: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
    pub timezone: Option<String>,
    #[sqlx(json(nullable))]
    pub discord_user: Option<DiscordUser>,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct DiscordUser {
    pub discord_id: i64,
    pub username: String,
    pub user_id: Uuid,
}
