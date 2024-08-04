use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    #[sqlx(rename = "user_id")]
    id: u32,
    pub username: String,
    pub created: DateTime<Utc>,
}

impl std::fmt::Debug for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("User")
            .field("id", &self.id)
            .field("username", &self.username)
            .field("password", &"*****")
            .field("created", &self.created.to_rfc3339())
            .finish()
    }
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct UserSession {
    #[sqlx(rename = "session_id")]
    id: u32,
    user_id: u32,
    created: DateTime<Utc>,
}
