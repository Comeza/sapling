use sqlx::query;
use sqlx::SqlitePool;
use tracing::info;

pub const SQL_LOGIN_USER: &str = include_str!("login_user.sql");
pub const SQL_REGISTER_USER: &str = include_str!("register_user.sql");

pub const SQL_FETCH_USER_SESSION: &str = include_str!("fetch_user_session.sql");
pub const SQL_FETCH_PRODUCT: &str = include_str!("fetch_product.sql");

pub const SQL_INSERT_SESSION: &str = include_str!("insert_user_session.sql");
pub const SQL_INSERT_PRODUCT: &str = include_str!("insert_product.sql");

pub async fn create_tables(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    info!("Setting up session table");
    query(include_str!("create_session_table.sql"))
        .execute(pool)
        .await?;

    info!("Setting up user table");
    query(include_str!("create_user_table.sql"))
        .execute(pool)
        .await?;

    info!("Creating product table");
    query(include_str!("create_product_table.sql"))
        .execute(pool)
        .await?;

    Ok(())
}
