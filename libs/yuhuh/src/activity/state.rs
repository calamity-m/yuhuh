use std::sync::Arc;

use sqlx::PgPool;

use crate::activity::{
    create_activity_entries::{
        CreateActivityEntriesRepository, CreateActivityEntriesRepositoryImpl,
    },
    read_activity_entries::{ReadActivityEntriesRepository, ReadActivityEntriesRepositoryImpl},
};

#[derive(Debug)]
pub struct ActivityState {
    pub create_activity_entries_repo: Arc<dyn CreateActivityEntriesRepository>,
    pub read_activity_entries_repo: Arc<dyn ReadActivityEntriesRepository>,
}

impl ActivityState {
    pub fn new(db: PgPool) -> Self {
        ActivityState {
            create_activity_entries_repo: Arc::new(CreateActivityEntriesRepositoryImpl::new(
                db.clone(),
            )),
            read_activity_entries_repo: Arc::new(ReadActivityEntriesRepositoryImpl::new(
                db.clone(),
            )),
        }
    }
}
