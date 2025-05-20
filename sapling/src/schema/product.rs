use async_graphql::{Context, Object, Result};
use sqlx::FromRow;

use crate::{
    Database,
    product::{Ean, EanValidator, Product, Item},
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
let product = match row { Some(row) => Some(Product::from_row(&row)?),
            None => None,
        };

        Ok(product)
    }

    async fn products<'a>(&self, ctx: &Context<'a>) -> Result<Vec<Product>> {
        let pool = ctx.data::<Database>()?;
        let rows = sqlx::query("SELECT * FROM product;")
            .fetch_all(pool)
            .await?
            .iter()
            .map(Product::from_row)
            .collect::<Result<Vec<Product>, _>>()?;

        Ok(rows)
    }

    async fn items<'a>(&self, ctx: &Context<'a>) -> Result<Vec<Item>> {
        let pool = ctx.data::<Database>()?;
        let rows = sqlx::query("SELECT * FROM item;")
            .fetch_all(pool)
            .await?
            .iter()
            .map(Item::from_row)
            .collect::<Result<Vec<Item>, _>>()?;

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

    async fn insert_item<'a>(&self, ctx: &'a Context<'a>, ean: Ean, cost: i32) -> Result<Item> {
        let pool = ctx.data::<Database>()?;
        let row = sqlx::query(queries::SQL_INSERT_ITEM).bind(ean).bind(cost).fetch_one(pool).await?;
        let stock = Item::from_row(&row)?;
        Ok(stock)
    }
}
