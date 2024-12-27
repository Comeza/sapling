use chrono::Utc;
use serde::Serialize;
use sqlx::prelude::FromRow;

#[derive(Debug, FromRow, Serialize)]
pub struct Session {
    pub token: String,

    #[serde(skip_serializing)]
    pub user_id: u32,

    pub created: chrono::DateTime<Utc>,
}
