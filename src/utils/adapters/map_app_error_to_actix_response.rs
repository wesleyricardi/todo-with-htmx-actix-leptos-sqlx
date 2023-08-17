use actix_web::HttpResponse;

use crate::error::app_error::{AppError, Code};

pub fn app_error_to_actix_response(error: AppError) -> HttpResponse {
    match error.code {
        Code::InvalidArgument => HttpResponse::BadRequest()
            .content_type("text/html; charset=utf-8")
            .body(error.message),
        Code::NotFound => HttpResponse::NotFound()
            .content_type("text/html; charset=utf-8")
            .body(error.message),
        Code::AlreadyExists => HttpResponse::Conflict()
            .content_type("text/html; charset=utf-8")
            .body(error.message),
        Code::PermissionDenied => HttpResponse::Forbidden()
            .content_type("text/html; charset=utf-8")
            .body(error.message),
        Code::Unauthenticated => HttpResponse::Unauthorized()
            .content_type("text/html; charset=utf-8")
            .body(error.message),
        Code::Internal => HttpResponse::InternalServerError()
            .content_type("text/html; charset=utf-8")
            .body("Internal error"),
        Code::Unknown => HttpResponse::InternalServerError()
            .content_type("text/html; charset=utf-8")
            .body("Unknown error"),
        Code::DatabaseError => HttpResponse::InternalServerError()
            .content_type("text/html; charset=utf-8")
            .body("Internal error"),
        Code::SQLError => HttpResponse::InternalServerError()
            .content_type("text/html; charset=utf-8")
            .body("Internal error"),
    }
}
