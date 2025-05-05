mod auth;
mod product;

use async_graphql::http::GraphiQLSource;
use axum::response::{Html, IntoResponse};

pub struct RootQuery;

pub struct RootMutation;

pub(crate) async fn graphiql() -> impl IntoResponse {
    Html(GraphiQLSource::build().endpoint("/gql").finish())
}
