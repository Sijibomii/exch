use actix::MailboxError;
use actix_web::{error, http, HttpResponse};
use data_encoding::DecodeError;
use diesel::result::{DatabaseErrorKind, Error as DieselError};
use jsonwebtoken::errors::Error as JwtError;
use openssl::error::ErrorStack;
use failure::Fail;
use serde_json::Error as SerdeError;
use serde_json::json;
use core::{db::Error as DbError, ModelError};


#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "{}", _0)]
    ModelError(#[cause] ModelError),
    #[fail(display = "{}", _0)]
    DecodeError(#[cause] DecodeError),
    #[fail(display = "{}", _0)]
    JwtError(#[cause] JwtError),
    #[fail(display = "{}", _0)]
    ErrorStack(#[cause] ErrorStack),
    #[fail(display = "{}", _0)]
    MailboxError(#[cause] MailboxError),
    #[fail(display = "incorrect password")]
    IncorrectPassword,
    #[fail(display = "{}", _0)]
    PayloadError(#[cause] error::PayloadError),
    #[fail(display = "unauthorized request")]
    UnAuthorizedRequestAccount,
    #[fail(display = "{}", _0)]
    SerdeError(#[cause] SerdeError),
    #[fail(display = "{}", _0)]
    BadRequest(&'static str),
    #[fail(display = "internal server error")]
    InternalServerError
}

impl error::ResponseError for Error {
    fn error_response(&self) -> HttpResponse {
        let user_err_message = format!("{}", json!({ "message": format!("{}", self) }));
        let server_err_message = format!("{}", json!({ "message": "internal server error" }));

        match *self {
            Error::BadRequest(_) | Error::IncorrectPassword  => {
                HttpResponse::build(http::StatusCode::BAD_REQUEST)
                    .body(user_err_message)
            }

            Error::UnAuthorizedRequestAccount => {
                HttpResponse::build(http::StatusCode::FORBIDDEN).body(user_err_message)
            }

            Error::ModelError(ref e) => match *e {
                ModelError::DbError(ref e) => match *e {
                    DbError::DieselError(ref e) => match *e {
                        DieselError::DatabaseError(ref kind, _) => match kind {
                            DatabaseErrorKind::UniqueViolation => {
                                HttpResponse::new(http::StatusCode::OK)
                            }
                            _ => HttpResponse::build(http::StatusCode::INTERNAL_SERVER_ERROR)
                                .body(server_err_message),
                        },
                        DieselError::NotFound => HttpResponse::build(http::StatusCode::NOT_FOUND)
                            .body(user_err_message),
                        _ => HttpResponse::build(http::StatusCode::INTERNAL_SERVER_ERROR)
                            .body(server_err_message),
                    },
                    _ => HttpResponse::build(http::StatusCode::INTERNAL_SERVER_ERROR)
                        .body(server_err_message),
                },
                _ => HttpResponse::build(http::StatusCode::INTERNAL_SERVER_ERROR)
                    .body(server_err_message),
            },

            _ => HttpResponse::build(http::StatusCode::INTERNAL_SERVER_ERROR)
                .body(server_err_message),
        }
    }
}

impl From<ModelError> for Error {
    fn from(e: ModelError) -> Error {
        Error::ModelError(e)
    }
}

impl From<DecodeError> for Error {
    fn from(e: DecodeError) -> Error {
        Error::DecodeError(e)
    }
}

impl From<ErrorStack> for Error {
    fn from(e: ErrorStack) -> Error {
        Error::ErrorStack(e)
    }
}

impl From<JwtError> for Error {
    fn from(e: JwtError) -> Error {
        Error::JwtError(e)
    }
}

impl From<MailboxError> for Error {
    fn from(e: MailboxError) -> Error {
        Error::MailboxError(e)
    }
}

impl From<error::PayloadError> for Error {
    fn from(e: error::PayloadError) -> Error {
        Error::PayloadError(e)
    }
}

impl From<SerdeError> for Error {
    fn from(e: SerdeError) -> Error {
        Error::SerdeError(e)
    }
}

