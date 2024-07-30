pub const CREATE_TABLES: &str = include_str!("create_tables.sql");

pub const INSERT_USER: &str = include_str!("insert_user.sql");

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
        let create_user = query(super::INSERT_USER).bind("comesa").execute(&mut db).await;

        assert!(create_user.is_ok());

        Ok(())
    }
}
