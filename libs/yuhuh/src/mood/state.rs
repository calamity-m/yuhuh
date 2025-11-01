use std::sync::Arc;

use sqlx::PgPool;

use crate::mood::{
    create_mood_entry::repository::{CreateMoodEntryRepository, CreateMoodEntryRepositoryImpl},
    read_mood_entries::repository::{ReadMoodEntriesRepository, ReadMoodEntriesRepositoryImpl},
};

#[derive(Debug)]
pub struct MoodState {
    pub create_mood_entry_repo: Arc<dyn CreateMoodEntryRepository>,
    pub read_mood_entries_repo: Arc<dyn ReadMoodEntriesRepository>,
}

impl MoodState {
    pub fn new(db: PgPool) -> Self {
        MoodState {
            create_mood_entry_repo: Arc::new(CreateMoodEntryRepositoryImpl::new(db.clone())),
            read_mood_entries_repo: Arc::new(ReadMoodEntriesRepositoryImpl::new(db.clone())),
        }
    }
}
