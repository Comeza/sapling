use async_graphql::{Context, Object, Result};
use sqlx::FromRow;

use crate::{
    Database,
    queries,
    session::{Session, User},
};

#[derive(Default)]
pub struct AuthMutation;

#[Object]
impl AuthMutation {
    async fn login<'a>(&self, ctx: &Context<'a>, username: String, password: String) -> Result<Session> {
        let db = ctx.data::<Database>()?;
        let row =
            sqlx::query(queries::SQL_LOGIN_USER).bind(username).fetch_optional(db).await?.ok_or("No user found")?;
        let user = User::from_row(&row)?;

        let is_valid = bcrypt::verify(password, &user.password)?;
        if !is_valid {
            Err("Invalid passowrd")?;
        }

        let token = uuid::Uuid::new_v4();

        let row = sqlx::query(queries::SQL_INSERT_SESSION)
            .bind(token.as_simple().to_string())
            .bind(user.user_id)
            .fetch_one(db)
            .await?;

        Ok(Session::from_row(&row)?)
    }

    async fn register<'a>(&self, ctx: &Context<'a>, username: String, password: String) -> Result<User> {
        let db = ctx.data::<Database>()?;
        let password = bcrypt::hash(password, 10)?;

        let row = sqlx::query(queries::SQL_REGISTER_USER).bind(&username).bind(&password).fetch_one(db).await?;
        let user = User::from_row(&row)?;

        Ok(user)
    }
}
