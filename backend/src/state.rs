use crate::database::connection::DbPool;

#[derive(Clone)]
pub struct AppState {
    pub pool: DbPool,
}
