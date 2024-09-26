use actix_web::{HttpResponse, ResponseError};
use deadpool_postgres::PoolError;
use derive_more::{Display, Error, From};
use tokio_pg_mapper::Error as PGMError;
use tokio_postgres::error::Error as PGError;

#[derive(Debug, Display, Error, From)]
pub enum AppError {
    NotFound,
    PGError(PGError),
    PGMError(PGMError),
    PoolError(PoolError),
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        match *self {
            AppError::NotFound => HttpResponse::NotFound().finish(),
            AppError::PoolError(ref err) => {
                HttpResponse::InternalServerError().body(err.to_string())
            }
            _ => HttpResponse::InternalServerError().finish(),
        }
    }
}