use std::path::Path;

use async_graphql::{EmptySubscription, Schema};
use async_graphql_axum::GraphQL;
use axum::{Router, routing::get};
use sqlx::{Pool, Sqlite, SqlitePool, sqlite::SqliteConnectOptions};
use thiserror::Error;
use tokio::net::TcpListener;
use tracing::info;

mod product;
mod queries;
mod schema;
mod session;

type Database = Pool<Sqlite>;

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

    let pool = setup_db("db.sqlite").await?;
    let listener = TcpListener::bind("0.0.0.0:3000").await?;

    let gql_schema = Schema::build(schema::RootQuery::default(), schema::RootMutation::default(), EmptySubscription)
        .data(pool)
        .finish();

    if cfg!(debug_assertions) {
        info!("Starting webserver on http://localhost:3000");
        info!("Starting gql endpoint on http://localhost:3000/gql");
    }

    #[rustfmt::skip]
    let router = Router::new()
        .route("/gql", get(schema::graphiql).post_service(GraphQL::new(gql_schema)));

    Ok(axum::serve(listener, router).await?)
}

async fn setup_db(path: impl AsRef<Path>) -> Result<Pool<Sqlite>, BackendError> {
    let options = SqliteConnectOptions::new().filename(path);

    if !options.get_filename().exists() {
        let pool = SqlitePool::connect_with(options.create_if_missing(true)).await?;

        info!("Creating core tables");
        sqlx::query(queries::SQL_CREATE_TABLES).execute(&pool).await?;

        info!("Creating auth tables");
        sqlx::query(queries::SQL_CREATE_AUTH_TABLES).execute(&pool).await?;
        return Ok(pool);
    } else {
        // TODO: Run migrations
        return Ok(SqlitePool::connect_with(options).await?);
    }
}
