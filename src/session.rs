use std::fmt::Debug;

use async_graphql::SimpleObject;
use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::prelude::FromRow;

#[derive(FromRow, Serialize, SimpleObject)]
pub struct User {
    pub user_id: u32,
    pub username: String,

    #[graphql(skip)]
    pub password: String,

    pub created: DateTime<Utc>,
}

impl Debug for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("User")
            .field("user_id", &self.user_id)
            .field("username", &self.username)
            .field("password", &"[REDECATD]")
            .field("created", &self.created)
            .finish()
    }
}

#[derive(Debug, FromRow, SimpleObject)]
pub struct Session {
    pub token: String,
    pub user_id: u32,
    pub created: DateTime<Utc>,
}
