use std::sync::Arc;

use sqlx::PgPool;

use crate::{
    food::find_food_entry::{FindFoodEntryRepository, FindFoodEntryRepositoryImpl},
    mood::{
        create_assignments::repository::{
            CreateAssignmentRepository, CreateAssignmentRepositoryImpl,
        },
        create_mood_entry::repository::{CreateMoodEntryRepository, CreateMoodEntryRepositoryImpl},
        read_assignments::repository::{ReadAssignmentRepository, ReadAssignmentRepositoryImpl},
    },
};

#[derive(Debug)]
pub struct MoodState {
    pub create_mood_entry_repo: Arc<dyn CreateMoodEntryRepository>,
    pub find_food_entry_repo: Arc<dyn FindFoodEntryRepository>,
    pub create_assignemnts_repo: Arc<dyn CreateAssignmentRepository>,
    pub read_assignments_repo: Arc<dyn ReadAssignmentRepository>,
}

impl MoodState {
    pub fn new(db: PgPool) -> Self {
        MoodState {
            create_mood_entry_repo: Arc::new(CreateMoodEntryRepositoryImpl::new(db.clone())),
            find_food_entry_repo: Arc::new(FindFoodEntryRepositoryImpl::new(db.clone())),
            create_assignemnts_repo: Arc::new(CreateAssignmentRepositoryImpl::new(db.clone())),
            read_assignments_repo: Arc::new(ReadAssignmentRepositoryImpl::new(db.clone())),
        }
    }
}
