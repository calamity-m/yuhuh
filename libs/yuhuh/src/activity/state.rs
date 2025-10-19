use std::sync::Arc;

use sqlx::PgPool;

use crate::{
    activity::create_activity_entries::repository::{
        CreateActivityEntriesRepository, CreateActivityEntriesRepositoryImpl,
    },
    food::find_food_entry::{FindFoodEntryRepository, FindFoodEntryRepositoryImpl},
    mood::create_mood_entry::repository::{
        CreateMoodEntryRepository, CreateMoodEntryRepositoryImpl,
    },
};

#[derive(Debug)]
pub struct ActivityState {
    pub create_activity_entries_repo: Arc<dyn CreateActivityEntriesRepository>,
}

impl ActivityState {
    pub fn new(db: PgPool) -> Self {
        ActivityState {
            create_activity_entries_repo: Arc::new(CreateActivityEntriesRepositoryImpl::new(
                db.clone(),
            )),
        }
    }
}
