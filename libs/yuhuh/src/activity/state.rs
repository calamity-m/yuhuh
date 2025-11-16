use std::sync::Arc;

use sqlx::PgPool;

use crate::activity::create_activity_entries::repository::{
    CreateActivityEntriesRepository, CreateActivityEntriesRepositoryImpl,
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
