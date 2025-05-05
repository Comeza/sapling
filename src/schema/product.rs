use async_graphql::{Context, Object, Result};
use sqlx::FromRow;

use crate::{
    Database,
    product::{Ean, EanValidator, Product},
    queries,
};

use super::RootQuery;

#[Object]
impl RootQuery {
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
}
