
use axum::{
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use sqlx::{prelude::*, sqlite::SqliteConnectOptions, SqlitePool};
use tracing::info;

mod paths;
mod queries;
mod user;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let subscriber = tracing_subscriber::fmt().compact().finish();
    tracing::subscriber::set_global_default(subscriber).unwrap();

    info!("Loading database file");
    let options = SqliteConnectOptions::new()
        .filename("db.sqlite")
        .create_if_missing(true);

    let pool = SqlitePool::connect_with(options).await?;

    info!("Creating tables");
    pool.execute(queries::CREATE_TABLES).await?;

    let app = Router::new()
        .route("/", get(root))
        .route("/users", get(paths::get_users))
        .route("/user/create", post(paths::create_user))
        .with_state(pool);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}

async fn root() -> impl IntoResponse {
    "Hey"
}
