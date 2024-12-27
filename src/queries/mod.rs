use tracing::info;
use sqlx::query;
use sqlx::Executor;
use sqlx::SqlitePool;
use thiserror::Error;

pub const SQL_LOGIN_USER: &str = include_str!("login_user.sql");
pub const SQL_REGISTER_USER: &str = include_str!("register_user.sql");

pub const SQL_FETCH_USER_SESSION: &str = include_str!("fetch_user_session.sql");

pub const SQL_INSERT_SESSION: &str = include_str!("insert_user_session.sql");

pub async fn create_tables(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    info!("Setting up session table");
    query(include_str!("create_session_table.sql"))
        .execute(pool)
        .await?;

    info!("Setting up user table");
    query(include_str!("create_user_table.sql"))
        .execute(pool)
        .await?;

    Ok(())
}
