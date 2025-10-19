use std::sync::Arc;

use sqlx::PgPool;

use crate::{
    activity::create_activity_entries::repository::{
        CreateActivityEntriesRepository, CreateActivityEntriesRepositoryImpl,
    },
    food::find_food_entry::{FindFoodEntryRepository, FindFoodEntryRepositoryImpl},
    mood::create_mood_entry::repository::{EnterMoodEntryRepository, EnterMoodEntryRepositoryImpl},
};

#[derive(Debug)]
pub struct MoodState {
    pub create_activity_entries_repo: Arc<dyn CreateActivityEntriesRepository>,
}

impl MoodState {
    pub fn new(db: PgPool) -> Self {
        MoodState {
            create_activity_entries_repo: Arc::new(CreateActivityEntriesRepositoryImpl::new(
                db.clone(),
            )),
        }
    }
}
