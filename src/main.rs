use sqlx::{prelude::*, sqlite::SqliteConnectOptions};
use tracing::info;

mod queries;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let subscriber = tracing_subscriber::fmt().compact().finish();
    tracing::subscriber::set_global_default(subscriber).unwrap();

    info!("Loading database file");
    let mut con = SqliteConnectOptions::new()
        .filename("db.sqlite")
        .create_if_missing(true)
        .connect()
        .await?;

    info!("Creating tables");
    con.execute(queries::CREATE_TABLES).await?;

    Ok(())
}
