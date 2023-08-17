use sqlx::Error;

use crate::error::app_error::{AppError, Code};

pub fn sqlx_error_to_app_error(error: Error) -> AppError {
    match error {
        Error::RowNotFound => AppError::new(Code::NotFound, "DB: nothing found with given parameters"),
        Error::Database(err) => match err.code().as_deref() {
            Some("23505") => AppError::new(Code::AlreadyExists, "DB: insert or update on table violates unique constraint"),
            Some("23514") => AppError::new(Code::DatabaseError, "insert or update on table violates check verification"),
            Some("23506") => AppError::new(Code::DatabaseError, "delete on table violates foreign key constraint"),
            Some("23503") => AppError::new(Code::DatabaseError, "insert or update on table violates foreign key constraint"),
            Some("23502") => AppError::new(Code::DatabaseError, "insert or update on table violates not null"),
            Some(code) => AppError::new(Code::DatabaseError, format!("error code: {}, message: {}", code, err.message())),
            None => AppError::new(Code::DatabaseError, "error unknown"),
        },
        Error::Configuration(_) => AppError::new(Code::SQLError, "Error occurred while parsing a connection string"),
        Error::Io(_) => AppError::new(Code::SQLError, "Error communicating with the database backend"),
        Error::Tls(_) => AppError::new(Code::SQLError, "Error occurred while attempting to establish a TLS connection"),
        Error::Protocol(_) => AppError::new(Code::SQLError, "Unexpected or invalid data encountered while communicating with the database"),
        Error::TypeNotFound { type_name: _ } => AppError::new(Code::SQLError, "Type in query doesn't exist. Likely due to typo or missing user type"),
        Error::ColumnIndexOutOfBounds { index: _, len: _ } => AppError::new(Code::SQLError, "Column index was out of bounds"),
        Error::ColumnNotFound(_) => AppError::new(Code::SQLError, "No column found for the given name"),
        Error::ColumnDecode { index: _, source: _ } => AppError::new(Code::SQLError, "Error occurred while decoding a value from a specific column"),
        Error::Decode(_) => AppError::new(Code::SQLError, "Error occurred while decoding a value from a specific column"),
        Error::PoolTimedOut => AppError::new(Code::SQLError, "A [Pool::acquire] timed out due to connections not becoming available or because another task encountered too many errors while trying to open a new connection"),
        Error::PoolClosed => AppError::new(Code::SQLError, "[Pool::close] was called while we were waiting in [Pool::acquire]"),
        Error::WorkerCrashed => AppError::new(Code::SQLError, "[Pool::close] was called while we were waiting in [Pool::acquire]"),
        Error::Migrate(_) => AppError::new(Code::SQLError, "migrate Error"),
        _ => AppError::new(Code::SQLError, "Error unknown"),
    }
}
