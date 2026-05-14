use crate::common::response;
use axum::{
    extract::rejection::JsonRejection,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("bad request: {0}")]
    BadRequest(String),
    #[error("not found: {0}")]
    NotFound(String),
    #[error("method not allowed")]
    MethodNotAllowed,
    #[error("internal server error")]
    Internal,
}

impl AppError {
    pub fn bad_request(message: impl Into<String>) -> Self {
        Self::BadRequest(message.into())
    }
    pub fn not_found(resource: impl Into<String>) -> Self {
        Self::NotFound(resource.into())
    }
    pub fn from_json_rejection(rejection: JsonRejection) -> Self {
        Self::bad_request(format!("Invalid JSON body: {}", rejection.body_text()))
    }

    fn status_code(&self) -> StatusCode {
        match self {
            Self::BadRequest(_) => StatusCode::BAD_REQUEST,
            Self::NotFound(_) => StatusCode::NOT_FOUND,
            Self::MethodNotAllowed => StatusCode::METHOD_NOT_ALLOWED,
            Self::Internal => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn code(&self) -> &'static str {
        match self {
            Self::BadRequest(_) => "BAD_REQUEST",
            Self::NotFound(_) => "NOT_FOUND",
            Self::MethodNotAllowed => "METHOD_NOT_ALLOWED",
            Self::Internal => "INTERNAL_ERROR",
        }
    }

    fn message(&self) -> String {
        match self {
            Self::BadRequest(message) => message.clone(),
            Self::NotFound(resource) => format!("{resource} not found"),
            Self::MethodNotAllowed => "Method not allowed".to_string(),
            Self::Internal => "Internal server error".to_string(),
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        response::error(self.status_code(), self.code(), self.message()).into_response()
    }
}

pub async fn not_found() -> AppError {
    AppError::not_found("Route")
}
pub async fn method_not_allowed() -> AppError {
    AppError::MethodNotAllowed
}
