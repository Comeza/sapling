use axum::{
    extract::Json,
    extract::{Query, State},
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};
use sqlx::{query, Executor};
use sqlx::{FromRow, SqlitePool};

use crate::{
    queries::{self, create_user_and_session},
    user::{User, UserSession},
};

use super::Paged;

#[derive(Serialize, Deserialize)]
pub struct CreateUserRequest {
    username: String,
    password: String,
}

pub async fn create_user(
    pool: State<SqlitePool>,
    req: Json<CreateUserRequest>,
) -> impl IntoResponse {
    let CreateUserRequest { username, password } = req.0;

    let row = create_user_and_session(username, password)
        .fetch_one(&*pool)
        .await
        .unwrap();

    let session = UserSession::from_row(&row).unwrap();

    Json(session)
}

pub async fn get_users(
    pool: State<SqlitePool>,
    offset: Option<Query<Paged<()>>>, // this does not work
) -> impl IntoResponse {
    let offset = offset.map(|j| j.offset()).unwrap_or_default();

    let users: Vec<User> = pool
        .fetch_all(queries::fetch_users(100, offset))
        .await
        .unwrap()
        .into_iter()
        .map(|row| User::from_row(&row).unwrap())
        .collect();

    Json(users)
}
