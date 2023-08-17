#[derive(Debug)]
pub enum Code {
    Unknown,
    InvalidArgument,
    NotFound,
    AlreadyExists,
    PermissionDenied,
    Internal,
    Unauthenticated,
    DatabaseError,
    SQLError,
}

pub struct AppError {
    pub code: Code,
    pub message: String,
}

impl AppError {
    pub fn new(code: Code, message: impl Into<String>) -> AppError {
        AppError {
            code,
            message: message.into(),
        }
    }
    pub fn unknown(message: impl Into<String>) -> AppError {
        Self {
            code: Code::Unknown,
            message: message.into(),
        }
    }
    pub fn invalid_argument(message: impl Into<String>) -> AppError {
        Self {
            code: Code::InvalidArgument,
            message: message.into(),
        }
    }
    pub fn not_found(message: impl Into<String>) -> AppError {
        Self {
            code: Code::NotFound,
            message: message.into(),
        }
    }
    pub fn already_exists(message: impl Into<String>) -> AppError {
        Self {
            code: Code::AlreadyExists,
            message: message.into(),
        }
    }
    pub fn permission_denied(message: impl Into<String>) -> AppError {
        Self {
            code: Code::PermissionDenied,
            message: message.into(),
        }
    }
    pub fn internal(message: impl Into<String>) -> AppError {
        Self {
            code: Code::Internal,
            message: message.into(),
        }
    }
    pub fn unauthenticated(message: impl Into<String>) -> AppError {
        Self {
            code: Code::Unauthenticated,
            message: message.into(),
        }
    }
    pub fn database_error(message: impl Into<String>) -> AppError {
        Self {
            code: Code::DatabaseError,
            message: message.into(),
        }
    }
    pub fn sql_error(message: impl Into<String>) -> AppError {
        Self {
            code: Code::SQLError,
            message: message.into(),
        }
    }
}
