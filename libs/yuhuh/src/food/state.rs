use std::sync::Arc;

use sqlx::PgPool;

use crate::food::{
    create_food_entries::{CreateFoodEntryRepository, CreateFoodEntryRepositoryImpl},
    find_food_entry::{FindFoodEntryRepository, FindFoodEntryRepositoryImpl},
};

#[derive(Debug)]
pub struct FoodState {
    pub create_food_entry_repo: Arc<dyn CreateFoodEntryRepository>,
    pub find_food_entry_repo: Arc<dyn FindFoodEntryRepository>,
}

impl FoodState {
    pub fn new(db: PgPool) -> Self {
        FoodState {
            create_food_entry_repo: Arc::new(CreateFoodEntryRepositoryImpl::new(db.clone())),
            find_food_entry_repo: Arc::new(FindFoodEntryRepositoryImpl::new(db.clone())),
        }
    }
}
