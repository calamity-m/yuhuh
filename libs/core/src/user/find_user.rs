pub mod handler {
    use std::{ops::Deref, sync::Arc};

    use axum::{
        Json,
        extract::{Path, State},
    };
    use tracing::{debug, info, info_span, instrument};
    use uuid::Uuid;

    use crate::{
        error::CoreError,
        user::{UserState, model::user::User},
    };

    #[instrument]
    pub async fn find_user_id(
        Path(id): Path<Uuid>,
        State(user_state): State<Arc<UserState>>,
    ) -> Result<Json<User>, CoreError> {
        debug!("entered find_user_id - {}", id);

        let res = user_state.find_user.find_user_id(id).await;

        match res {
            Ok(user) => {
                // dunno if this will work is needed?

                info_span!("find_user_Id", user=?user);
                info!("found user");
                todo!("need to figure this out - spans");

                Ok(Json(user))
            }
            Err(err) => Err({
                todo!("need to figure this out - err wrapping");
                err
            }),
        }
    }

    #[instrument]
    pub async fn find_user_uname(Path(username): Path<String>) -> Result<Json<User>, CoreError> {
        debug!("entered find_user_uname - {}", username);

        Err(CoreError::NotImplemented)
    }
}

pub mod service {
    use sqlx::PgPool;
    use uuid::Uuid;

    use crate::{error::CoreError, user::model::user::User};

    #[derive(Clone, Debug)]
    pub struct FindUserService {
        pub pool: PgPool,
    }

    impl FindUserService {
        pub fn new(pool: PgPool) -> Self {
            FindUserService { pool }
        }

        pub async fn find_user_id(&self, id: Uuid) -> Result<User, CoreError> {
            Err(CoreError::NotImplemented)
        }
    }
}
