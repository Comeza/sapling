use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::Deserialize;
use spore::{Ean, Product};
use sqlx::{query, FromRow};
use thiserror::Error;
use tracing::{error, info};

use crate::{queries, state::AppState};

#[derive(Debug, Deserialize)]
pub struct InsertProductRequest {
    ean: Ean,
    product_name: String,
    common_name: Option<String>,
}

#[derive(Debug, Error)]
pub enum InsertProductError {
    #[error("Invalid EAN")]
    InvalidEan,

    #[error("Invalid product name")]
    InvalidProductName,

    #[error("Sqlx: {0}")]
    Sqlx(#[from] sqlx::Error),
}

impl IntoResponse for InsertProductError {
    fn into_response(self) -> axum::response::Response {
        type E = InsertProductError;
        match self {
            E::InvalidEan | E::InvalidProductName => (StatusCode::BAD_REQUEST, self.to_string()).into_response(),
            E::Sqlx(error) => {
                error!("Sqlx error during product registration: {error}");
                StatusCode::INTERNAL_SERVER_ERROR.into_response()
            }
        }
    }
}

pub async fn delete_product() -> impl IntoResponse {
    "not yet implemented"
}

pub async fn insert_product(
    state: State<AppState>,
    json: Json<InsertProductRequest>,
) -> Result<Json<Product>, InsertProductError> {
    let ean = json.ean;

    if !ean.is_valid() {
        return Err(InsertProductError::InvalidEan);
    }

    if json.0.product_name.trim().is_empty() {
        return Err(InsertProductError::InvalidProductName);
    }

    let product = Product::new(ean, json.product_name.trim()).with_common_name(json.0.common_name);

    query(queries::SQL_INSERT_PRODUCT)
        .bind(&product.ean)
        .bind(&product.product_name)
        .bind(&product.common_name)
        .execute(&state.pool)
        .await?;

    Ok(Json(product))
}

#[derive(Debug, Error)]
pub enum FetchProductError {
    #[error("sqlx: {0}")]
    Sqlx(#[from] sqlx::Error),

    #[error("Product not found")]
    NotFound,
}

impl IntoResponse for FetchProductError {
    fn into_response(self) -> axum::response::Response {
        use FetchProductError as E;
        match self {
            E::Sqlx(error) => {
                error!("Sqlx error during product registration: {error}");
                StatusCode::INTERNAL_SERVER_ERROR.into_response()
            }
            E::NotFound => StatusCode::NOT_FOUND.into_response(),
        }
    }
}

pub async fn fetch_product(ean: Path<Ean>, state: State<AppState>) -> Result<Json<Product>, FetchProductError> {
    let product = query(queries::SQL_FETCH_PRODUCT)
        .bind(&ean.0)
        .fetch_optional(&state.pool)
        .await?
        .ok_or(FetchProductError::NotFound)?;

    Ok(Json(Product::from_row(&product)?))
}
