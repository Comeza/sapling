use sqlx::{query, Database};
use sqlx::{query::Query, Sqlite};

/// Helper type for sqlite queries
type SqliteQuery<'a> = Query<'a, Sqlite, <Sqlite as Database>::Arguments<'a>>;

pub const CREATE_TABLES: &str = include_str!("create_tables.sql");
pub const INSERT_USER: &str = include_str!("insert_user.sql");
pub const FETCH_USERS: &str = include_str!("fetch_users.sql");
pub const CREATE_SESSION: &str = include_str!("create_session.sql");
pub const CREATE_USER_AND_SESSOIN: &str = include_str!("create_user_and_session.sql");

pub fn fetch_users<'a>(limit: u32, offset: u32) -> SqliteQuery<'a> {
    query(FETCH_USERS).bind(limit).bind(offset)
}

pub fn create_session<'a>(user_id: u32) -> SqliteQuery<'a> {
    query(CREATE_SESSION).bind(user_id)
}

// TODO: HASH password

pub fn create_user_and_session<'a>(username: String, password: String) -> SqliteQuery<'a> {
    query(CREATE_USER_AND_SESSOIN).bind(username).bind(password)
}

#[cfg(test)]
mod tests {
    use sqlx::{query, Connection, SqliteConnection};

    pub async fn setup_db() -> SqliteConnection {
        let mut con = SqliteConnection::connect("sqlite::memory:").await.unwrap();
        query(super::CREATE_TABLES).execute(&mut con).await.unwrap();
        con
    }

    #[tokio::test]
    pub async fn create_db() {
        let mut con = SqliteConnection::connect("sqlite::memory:").await.unwrap();
        let res = sqlx::query(super::CREATE_TABLES).execute(&mut con).await;
        assert!(res.is_ok());
    }

    #[tokio::test]
    pub async fn create_user() -> Result<(), sqlx::Error> {
        let mut db = setup_db().await;
        let create_user = query(super::INSERT_USER)
            .bind("comesa")
            .execute(&mut db)
            .await;

        assert!(create_user.is_ok());

        Ok(())
    }
}
