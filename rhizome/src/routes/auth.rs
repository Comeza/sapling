use crate::auth::session::Session;
use crate::queries;
use crate::state::AppState;
use crate::user::User;
use axum::http::StatusCode;
use axum::response::Response;
use axum::{extract::State, response::IntoResponse, Json};
use tracing::error;
use serde::{Deserialize, Serialize};
use sqlx::{query, FromRow};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum RegistrationError {
    #[error("User already exists")]
    UserExists,
    #[error("Password has invalid format")]
    InvalidPassword,
    #[error("Sqlx: {0}")]
    Sqlx(#[from] sqlx::Error),
}

impl IntoResponse for RegistrationError {
    fn into_response(self) -> Response {
        match self {
            RegistrationError::UserExists | RegistrationError::InvalidPassword => {
                (StatusCode::BAD_REQUEST, self.to_string()).into_response()
            },
            RegistrationError::Sqlx(err) => {
                error!("Registration caused sql error: {err}");
                StatusCode::INTERNAL_SERVER_ERROR.into_response()
            }
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct RegisterRequest {
    username: String,
    password: String,
    // TODO: add email or smth for 2fa
}

pub async fn register(
    state: State<AppState>,
    body: Json<RegisterRequest>,
) -> Result<Json<Session>, RegistrationError> {
    // TODO: Use regex
    if body.password.len() <= 5 {
        return Err(RegistrationError::InvalidPassword);
    }

    let password = bcrypt::hash(&body.password, 10).unwrap();
    let res = query(queries::SQL_REGISTER_USER)
        .bind(&body.username)
        .bind(&password)
        .fetch_one(&state.pool)
        .await;

    if let Err(sqlx::Error::Database(db_error)) = &res {
        if db_error.is_unique_violation() {
            return Err(RegistrationError::UserExists);
        }
    }

    Ok(Json(Session::from_row(&res?)?))
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Error, Debug)]
pub enum LoginError {
    #[error("User does not exist")]
    InvalidUser,

    #[error("Invalid Password")]
    InvalidPassword,

    #[error("Sqlx: {0}")]
    Sqlx(#[from] sqlx::Error),

    #[error("Bcrypt: {0}")]
    Bcrypt(#[from] bcrypt::BcryptError),
}

impl IntoResponse for LoginError {
    fn into_response(self) -> Response {
        match self {
            LoginError::InvalidUser | LoginError::InvalidPassword => {
                (StatusCode::FORBIDDEN, self.to_string()).into_response()
            }
            LoginError::Sqlx(err) => {
                error!("Login caused sql error: {err}");
                StatusCode::INTERNAL_SERVER_ERROR.into_response()
            }
            LoginError::Bcrypt(err) => {
                error!("Login caused bcrypt error: {err}");
                StatusCode::INTERNAL_SERVER_ERROR.into_response()
            }
        }
    }
}

pub async fn login(
    state: State<AppState>,
    body: Json<LoginRequest>,
) -> Result<Json<Session>, LoginError> {
    let res = query(queries::SQL_LOGIN_USER)
        .bind(&body.username)
        .fetch_one(&state.pool)
        .await;

    if let Err(sqlx::Error::RowNotFound) = res {
        return Err(LoginError::InvalidUser);
    }

    let user = User::from_row(&res?)?;

    let password_valid = bcrypt::verify(&body.password, user.hashed_password.as_str())?;
    if !password_valid {
        return Err(LoginError::InvalidPassword);
    }

    let row = query(queries::SQL_INSERT_SESSION)
        .bind(user.id)
        .fetch_one(&state.pool)
        .await?;

    Ok(Json(Session::from_row(&row)?))
}
