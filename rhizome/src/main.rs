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
    let subscriber = tracing_subscriber::fmt().compact().finish();
    tracing::subscriber::set_global_default(subscriber).unwrap();

    let options = SqliteConnectOptions::new().filename("db.sqlite").create_if_missing(true);
    let pool = SqlitePool::connect_with(options).await?;

    queries::create_tables(&pool).await?;

    let product_router = Router::new()
        .route("/product", routing::post(routes::insert_product))
        .route("/product/:ean", routing::delete(routes::delete_product).get(routes::fetch_product));

    #[rustfmt::skip]
    let auth_router = Router::new()
        .route("/register", routing::post(routes::register))
        .route("/login", routing::post(routes::login));

    let listener = TcpListener::bind("0.0.0.0:3000").await?;

    #[rustfmt::skip]
    let router = Router::new()
        .nest("/auth/", auth_router)
        .nest("/api/", product_router)
        .with_state(AppState { pool });

    Ok(axum::serve(listener, router).await?)
}
