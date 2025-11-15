use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::{
    error::{ConversionError, RatingError},
    mood::rating::Rating,
};

#[derive(Debug, Serialize, Deserialize, ToSchema, sqlx::FromRow)]
pub struct MoodEntry {
    pub mood_record_id: Option<Uuid>,
    pub user_id: Uuid,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
    pub mood: Option<Rating>,
    pub energy: Option<Rating>,
    pub sleep: Option<Rating>,
    pub notes: Option<String>,
}

// =============================================================================
// Row Structs
// =============================================================================
#[derive(Debug, sqlx::FromRow)]
pub struct MoodEntryRow {
    pub mood_record_id: Option<Uuid>,
    pub user_id: Uuid,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
    pub mood: Option<i16>,
    pub energy: Option<i16>,
    pub sleep: Option<i16>,
    pub notes: Option<String>,
}

impl From<MoodEntry> for MoodEntryRow {
    fn from(value: MoodEntry) -> Self {
        MoodEntryRow {
            mood_record_id: value.mood_record_id,
            user_id: value.user_id,
            created_at: value.created_at,
            updated_at: value.updated_at,
            mood: value.mood.map(|r| r.get() as i16),
            energy: value.energy.map(|r| r.get() as i16),
            sleep: value.sleep.map(|r| r.get() as i16),
            notes: value.notes,
        }
    }
}

impl TryInto<MoodEntry> for MoodEntryRow {
    type Error = ConversionError;

    fn try_into(self) -> Result<MoodEntry, Self::Error> {
        let r = MoodEntry {
            mood_record_id: self.mood_record_id,
            user_id: self.user_id,
            created_at: self.created_at,
            updated_at: self.updated_at,
            mood: self
                .mood
                .map(Rating::try_from)
                .transpose()
                .map_err(|e| ConversionError::new(format!("failed to parse mood - {}", e)))?,
            energy: self
                .energy
                .map(Rating::try_from)
                .transpose()
                .map_err(|e| ConversionError::new(format!("failed to parse energy - {}", e)))?,
            sleep: self
                .sleep
                .map(Rating::try_from)
                .transpose()
                .map_err(|e| ConversionError::new(format!("failed to parse sleep - {}", e)))?,
            notes: self.notes,
        };

        Ok(r)
    }
}
