use async_graphql::{Context, Object, Result};
use sqlx::FromRow;

use crate::{
    Database,
    product::{Ean, EanValidator, Product},
    queries,
};

#[derive(Default)]
pub struct ProductQuery;

#[Object]
impl ProductQuery {
    async fn product<'a>(
        &self,
        ctx: &Context<'a>,
        #[graphql(validator(custom = "EanValidator"))] ean: Ean,
    ) -> Result<Option<Product>> {
        let pool = ctx.data::<Database>()?;
        let row = sqlx::query(queries::SQL_FETCH_PRODUCT).bind(&ean).fetch_optional(pool).await?;

        let product = match row {
            Some(row) => Some(Product::from_row(&row)?),
            None => None,
        };

        Ok(product)
    }

    async fn products<'a>(&self, ctx: &Context<'a>) -> Result<Vec<Product>> {
        let pool = ctx.data::<Database>()?;
        let rows = sqlx::query("SELECT * FROM product LIMIT 1000;")
            .fetch_all(pool)
            .await?
            .iter()
            .map(Product::from_row)
            .collect::<Result<Vec<Product>, _>>()?;

        Ok(rows)
    }
}

#[derive(Default)]
pub struct ProductMutation;

#[Object]
impl ProductMutation {
    async fn insert_product<'a>(
        &self,
        ctx: &'a Context<'a>,
        #[graphql(validator(custom = EanValidator))] ean: Ean,
        name: String,
    ) -> Result<Product> {
        let pool = ctx.data::<Database>()?;
        let row = sqlx::query(queries::SQL_INSERT_PRODUCT).bind(ean).bind(name).fetch_one(pool).await?;
        let product = Product::from_row(&row)?;
        Ok(product)
    }
}
