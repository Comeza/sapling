mod auth;
mod product;

use async_graphql::{MergedObject, http::GraphiQLSource};
use auth::AuthMutation;
use axum::response::{Html, IntoResponse};
use product::{ProductMutation, ProductQuery};

#[derive(Default, MergedObject)]
pub struct RootQuery(ProductQuery);

#[derive(Default, MergedObject)]
pub struct RootMutation(AuthMutation, ProductMutation);

pub(crate) async fn graphiql() -> impl IntoResponse {
    Html(GraphiQLSource::build().endpoint("/gql").finish())
}
