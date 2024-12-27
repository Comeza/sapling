pub mod session;

use crate::{queries, user::User, AppState};
use axum::{
    async_trait,
    extract::{FromRequest, Request},
    http::{header, StatusCode},
    response::IntoResponse,
};
use axum::response::Response;
use tracing::log::{error, log};
use sqlx::{query, FromRow};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AuthError {
    #[error("Empty Header")]
    MissingHeader,

    #[error("Invalid Header")]
    InvalidHeader,

    #[error("Invalid session token")]
    InvalidSessionToken,

    #[error("Sqlx: {0}")]
    Sqlx(#[from] sqlx::Error),
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        if let AuthError::Sqlx(err) = &self {
            error!("Error in Auth request: {err}");
        }

        if let AuthError::Sqlx(err) = self {
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }

        StatusCode::UNAUTHORIZED.into_response()
    }
}

#[derive(Debug)]
pub struct Auth(pub User);

#[async_trait]
impl FromRequest<AppState> for Auth {
    type Rejection = AuthError;

    async fn from_request(req: Request, state: &AppState) -> Result<Self, Self::Rejection> {
        let bearer = req
            .headers()
            .get(header::AUTHORIZATION)
            .ok_or(AuthError::MissingHeader)?
            .to_str()
            .map_err(|_| AuthError::InvalidHeader)?;

        if !bearer.starts_with("Bearer ") {
            return Err(AuthError::InvalidHeader);
        }

        let token = bearer.split_whitespace().nth(1).unwrap().trim();

        if token.is_empty() {
            return Err(AuthError::InvalidHeader);
        }

        let row = query(queries::SQL_FETCH_USER_SESSION)
            .bind(token)
            .bind(&token)
            .fetch_one(&state.pool)
            .await;

        if let Err(sqlx::Error::ColumnNotFound(_)) = row {
            return Err(AuthError::InvalidSessionToken);
        }

        Ok(Auth(User::from_row(&row?)?))
    }
}
