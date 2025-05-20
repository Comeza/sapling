use async_graphql::{ComplexObject, Context, CustomValidator, InputValueError, SimpleObject, scalar};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use crate::{Database, queries};

type Id = u32;

#[derive(Debug, FromRow, SimpleObject)]
#[graphql(complex)]
pub struct Product {
    ean: Ean,
    name: String,
    brand_id: Option<Id>,
    description: Option<String>,
    inserted_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

#[derive(Debug, FromRow)]
pub(crate) struct ProductTag {
    ean: Ean,
    tag_id: Id,
    created_at: DateTime<Utc>,
}

#[derive(Debug, FromRow, SimpleObject)]
pub struct Tag {
    tag_id: Id,
    name: String,
}

pub struct Brand {
    brand_id: Id,
    name: String,
    homepage: Option<String>,
    wikipedia: Option<String>,
}

#[derive(FromRow, SimpleObject)]
#[graphql(complex)]
pub struct Item {
    item_id: Id,
    ean: Ean,
    created_at: DateTime<Utc>,
    cost: Option<i32>,
}

#[ComplexObject]
impl Product {
    async fn tags<'a>(&self, ctx: &Context<'a>) -> async_graphql::Result<Vec<Tag>> {
        let pool = ctx.data::<Database>()?;
        let rows = sqlx::query(queries::SQL_FETCH_PRODUCT_TAGS).bind(self.ean).fetch_all(pool).await?;
        let tags = rows.iter().map(Tag::from_row).collect::<Result<Vec<_>, _>>()?;

        Ok(tags)
    }

    async fn groups<'a>(&self, ctx: &Context<'a>) -> async_graphql::Result<Vec<Tag>> {
        let pool = ctx.data::<Database>()?;
        let rows = sqlx::query(queries::SQL_FETCH_PRODUCT_GROUPS).bind(self.ean).fetch_all(pool).await?;
        let tags = rows.iter().map(Tag::from_row).collect::<Result<Vec<_>, _>>()?;

        Ok(tags)
    }

    async fn items<'a>(&self, ctx: &Context<'a>) -> async_graphql::Result<Vec<Item>> {
        let pool = ctx.data::<Database>()?;
        let rows = sqlx::query("SELECT * FROM item WHERE ean = ?;").bind(self.ean).fetch_all(pool).await?;
        let items = rows.iter().map(Item::from_row).collect::<Result<Vec<_>, _>>()?;
        
        Ok(items)
    }
}

#[ComplexObject]
impl Item {
    async fn product<'a>(&self, ctx: &Context<'a>) -> async_graphql::Result<Product> {
        let pool = ctx.data::<Database>()?;
        let row = sqlx::query(queries::SQL_FETCH_PRODUCT).bind(self.ean).fetch_one(pool).await?;
        let product = Product::from_row(&row)?;

        Ok(product)
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
