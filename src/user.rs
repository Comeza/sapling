use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::prelude::FromRow;

#[derive(Debug, FromRow, Serialize)]
pub struct User {
    #[sqlx(rename = "user_id")]
    pub id: u32,
    pub username: String,
    pub hashed_password: String,
    pub created: DateTime<Utc>,
}
