use async_graphql::{CustomValidator, InputValueError, OutputType, ScalarType, SimpleObject, Value, scalar};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

type Id = i32;

#[derive(Debug, FromRow, SimpleObject)]
pub struct Product {
    ean: Ean,
    name: String,
    //descripton: Option<String>,
    inserted_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

pub struct ProductGroup {
    group_id: Id,
    name: String,
}

pub struct Brand {
    brand_id: Id,
    name: String,
}

#[derive(FromRow)]
pub struct Stock {
    stock_id: u64,
    product: Id,
    amount: u32,
    inserted: DateTime<Utc>,
    last_used: Option<DateTime<Utc>>,
    cost: Option<i32>,
}

impl Product {
    pub fn new(ean: Ean, product_name: impl Into<String>) -> Self {
        todo!()
    }
}

#[derive(sqlx::Type, Debug, Serialize, Deserialize, Clone, Copy)]
#[sqlx(transparent)]
/// EAN (European Article Number) are _unique_ identification numbers for Products produced in the
/// EU.
///
/// There are two types of EAN. EAN13 (13 numbers) and EAN8 (8 numbers).
/// We need log2(10^14 - 1) ~ 47 Bits to enocde an EAN.
///
/// However SQLite does not support u64 integers out-of-the-box. Thus we will use an i64, wich is
/// supported. The most significant bits are unused, thus there won't be negative EANs.
pub struct Ean(pub i64);
scalar!(Ean, "EAN");

impl Ean {
    pub fn is_valid(&self) -> bool {
        let ean = self.0;
        let mut sum = 0;
        let mut digits = ean;
        let mut count = 0;

        while digits > 0 {
            let digit = digits % 10;
            sum += digit + digit * (count % 2 != 0) as i64 * 2;
            digits /= 10;
            count += 1;
        }

        sum % 10 == 0
    }
}

pub struct EanValidator;
impl CustomValidator<Ean> for EanValidator {
    fn check(&self, value: &Ean) -> Result<(), async_graphql::InputValueError<Ean>> {
        match value.is_valid() {
            true => Ok(()),
            false => Err(InputValueError::custom(format!("checksum is not valid"))),
        }
    }
}

#[cfg(test)]
mod test {
    use super::Ean;

    #[test]
    fn valid_ean13() {
        assert!(Ean(4029764001807).is_valid());
    }

    #[test]
    fn invalid_ean13() {
        assert!(!Ean(4029764001808).is_valid());
    }
}
