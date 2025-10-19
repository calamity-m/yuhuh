use std::sync::Arc;

use sqlx::PgPool;

use crate::{
    food::find_food_entry::{FindFoodEntryRepository, FindFoodEntryRepositoryImpl},
    mood::create_mood_entry::repository::{EnterMoodEntryRepository, EnterMoodEntryRepositoryImpl},
};

#[derive(Debug)]
pub struct MoodState {
    pub create_mood_entry_repo: Arc<dyn EnterMoodEntryRepository>,
    pub find_food_entry_repo: Arc<dyn FindFoodEntryRepository>,
}

impl MoodState {
    pub fn new(db: PgPool) -> Self {
        MoodState {
            create_mood_entry_repo: Arc::new(EnterMoodEntryRepositoryImpl::new(db.clone())),
            find_food_entry_repo: Arc::new(FindFoodEntryRepositoryImpl::new(db.clone())),
        }
    }
}
