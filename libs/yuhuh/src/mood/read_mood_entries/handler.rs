//! read mood entries HTTP handler
//!
//! This module provides HTTP endpoints for reading mood entries.

// ============================================================================
// Request/Response Types
// ============================================================================

use std::sync::Arc;

use axum::{
    Json,
    extract::{Query, State},
    http::StatusCode,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tracing::{debug, instrument};
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;

use crate::{
    error::YuhuhError,
    food::{model::FoodEntry, state::FoodState},
};

// ============================================================================
// HTTP Request types
// ============================================================================

/// Request parameters for finding mood entries.
#[derive(Debug, Deserialize, IntoParams)]
pub struct ReadMoodEntriesRequest {
    /// user ID to search by.
    pub user_id: Uuid,
    pub limit: Option<u32>,
    pub offset: Option<u32>,
    pub logged_before_date: Option<DateTime<Utc>>,
    pub logged_after_date: Option<DateTime<Utc>>,
}
