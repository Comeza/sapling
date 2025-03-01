use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(sqlx::Type, Debug, Serialize, Deserialize, Clone)]
#[sqlx(transparent)]
pub struct Ean(pub String);

impl Ean {
    pub fn is_valid(&self) -> bool {
        true // TODO: Check checksum
    }
}

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct Product {
    pub ean: Ean,

    // Sometimes procuts have a very long name that nobody uses
    pub product_name: String,
    pub common_name: Option<String>,
}

impl Product {
    pub fn new(ean: Ean, product_name: impl Into<String>) -> Self {
        Self {
            ean,
            product_name: product_name.into(),
            common_name: None,
        }
    }

    pub fn with_common_name<T: Into<String>>(self, common_name: Option<T>) -> Self {
        Self {
            common_name: common_name.map(T::into),
            ..self
        }
    }
}
