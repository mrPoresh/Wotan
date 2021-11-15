use actix_web::{error::ResponseError, HttpResponse};
use diesel::result::Error as DBError;
use std::convert::From;
use thiserror::Error;


// Errors handling
#[derive(Debug, Error, Serialize)]
pub enum ServiceError {

    #[error("Internal Server Error")]
    InternalServerError,

    #[error("BadRequest: {0}")]
    BadRequest(String),

    #[error("Unauthorized")]
    Unauthorized,

    #[error("Unable to connect to DB")]
    UnableToConnectToDb,

}

// Trait to convert errors into http respones with errors data
impl ResponseError for ServiceError {

    fn error_response(&self) -> HttpResponse {

        match self {

            ServiceError::InternalServerError => {
                HttpResponse::InternalServerError().json("Internal Server Error, Please try later")
            }
            ServiceError::UnableToConnectToDb => HttpResponse::InternalServerError()
                .json("Unable to connect to DB, Please try later"),
            ServiceError::BadRequest(ref message) => HttpResponse::BadRequest().json(message),
            ServiceError::Unauthorized => HttpResponse::Unauthorized().json("Unauthorized"),

        }

    }

}

// Return early handlers if UUID provided by the user is not valid
impl From<uuid::parser::ParseError> for ServiceError {

    fn from(_: uuid::parser::ParseError) -> ServiceError {

        ServiceError::BadRequest("Invalid UUID".into())

    }

}

// For Database
impl From<DBError> for ServiceError {

    fn from(error: DBError) -> ServiceError {

        match error {

            DBError::DatabaseError(_kind, info) => {
                let message = info.details().unwrap_or_else(|| info.message()).to_string();
                ServiceError::BadRequest(message)
            }
            _ => ServiceError::InternalServerError,
        }

    }

}

pub type ServiceResult<V> = std::result::Result<V, crate::errors::ServiceError>;