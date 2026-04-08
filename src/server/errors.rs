use std::fmt;

use axum::{http::StatusCode, response::IntoResponse};

#[derive(Debug)]
pub enum AppError {
    // NotFound,
    // InternalServerError,
    BadRequest,
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let (status, body) = match self {
            // AppError::NotFound => (StatusCode::NOT_FOUND, "Not Found".to_string()),
            // AppError::InternalServerError => (
            //     StatusCode::INTERNAL_SERVER_ERROR,
            //     "Internal Server Error".to_string(),
            // ),
            AppError::BadRequest => (StatusCode::BAD_REQUEST, "Bad request".to_string()),
        };
        (status, body).into_response()
    }
}
