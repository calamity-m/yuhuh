use std::sync::Arc;

use sqlx::PgPool;

use crate::food::{
    create_food_entries::{CreateFoodEntryRepository, CreateFoodEntryRepositoryImpl},
    read_food_entries::{FindFoodEntryRepository, FindFoodEntryRepositoryImpl},
};

#[derive(Debug)]
pub struct FoodState {
    pub create_food_entries_repo: Arc<dyn CreateFoodEntryRepository>,
    pub read_food_entries_repo: Arc<dyn FindFoodEntryRepository>,
}

impl FoodState {
    pub fn new(db: PgPool) -> Self {
        FoodState {
            create_food_entries_repo: Arc::new(CreateFoodEntryRepositoryImpl::new(db.clone())),
            read_food_entries_repo: Arc::new(FindFoodEntryRepositoryImpl::new(db.clone())),
        }
    }
}
