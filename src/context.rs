use sqlx::SqlitePool;

#[derive(Clone)]
pub struct AppContext {
    pub db_pool: SqlitePool,
}

impl AppContext {
    pub fn new(db_pool: SqlitePool) -> Self {
        AppContext { db_pool }
    }
}
