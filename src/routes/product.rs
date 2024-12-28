use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use sqlx::query;
use thiserror::Error;
use tracing::error;

use crate::{
    product::{self, Ean, Product},
    queries,
    state::AppState,
};

#[derive(Debug, Deserialize)]
pub struct InsertProductRequest {
    ean: String,
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
            E::InvalidEan | E::InvalidProductName => {
                (StatusCode::BAD_REQUEST, self.to_string()).into_response()
            }
            E::Sqlx(error) => {
                error!("Sqlx error during product registration: {error}");
                StatusCode::INTERNAL_SERVER_ERROR.into_response()
            }
        }
    }
}

pub async fn insert_product(
    state: State<AppState>,
    json: Json<InsertProductRequest>,
) -> Result<Json<Product>, InsertProductError> {
    let ean = Ean(json.ean.clone());

    if !ean.is_valid() {
        return Err(InsertProductError::InvalidEan);
    }

    if json.0.product_name.trim().is_empty() {
        return Err(InsertProductError::InvalidProductName);
    }

    let product = Product {
        ean,
        product_name: json.product_name.trim().to_owned(),
        common_name: json.0.common_name,
    };

    query(queries::SQL_INSERT_PRODUCT)
        .bind(&product.ean)
        .bind(&product.product_name)
        .bind(&product.common_name)
        .execute(&state.pool)
        .await?;

    Ok(Json(product))
}
