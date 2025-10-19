//! create mood entries HTTP handler
//!
//! This module provides HTTP endpoints for creating mood entries.

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
