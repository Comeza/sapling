use serde::{Deserialize, Serialize};
use sqlx::FromRow;

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

#[cfg(test)]
mod test {
    use crate::Ean;

    #[test]
    fn valid_ean13() {
        assert!(Ean(4029764001807).is_valid());
    }

    #[test]
    fn invalid_ean13() {
        assert!(!Ean(4029764001808).is_valid());
    }
}
