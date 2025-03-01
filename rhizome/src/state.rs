use sqlx::SqlitePool;

#[derive(Debug, Clone)]
pub struct AppState {
    pub pool: SqlitePool,
}
