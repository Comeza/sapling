use axum::{
    routing::{self},
    Router,
};
use sqlx::{sqlite::SqliteConnectOptions, SqlitePool};
use state::AppState;
use thiserror::Error;
use tokio::net::TcpListener;

mod auth;
mod queries;
mod routes;
mod state;
mod user;

#[derive(Debug, Error)]
pub enum BackendError {
    #[error("SqlxError: {0}")]
    Sqlx(#[from] sqlx::Error),

    #[error("IOError: {0}")]
    Io(#[from] std::io::Error),
}

#[tokio::main]
async fn main() -> Result<(), BackendError> {
    tracing_subscriber::fmt().init();

    let options = SqliteConnectOptions::new()
        .filename("db.sqlite")
        .create_if_missing(true);
    let pool = SqlitePool::connect_with(options).await?;

    queries::create_tables(&pool).await?;

    let listener = TcpListener::bind("0.0.0.0:3000").await?;
    let router = Router::new()
        .route("/auth/register", routing::post(routes::register))
        .route("/auth/login", routing::post(routes::login))
        .with_state(AppState { pool });

    Ok(axum::serve(listener, router).await?)
}
