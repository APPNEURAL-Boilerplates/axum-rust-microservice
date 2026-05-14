use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct SuccessResponse<T: Serialize> {
    pub ok: bool,
    pub data: T,
}

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub ok: bool,
    pub error: ErrorBody,
}

#[derive(Debug, Serialize)]
pub struct ErrorBody {
    pub code: &'static str,
    pub message: String,
}

pub fn json<T: Serialize>(status: StatusCode, data: T) -> impl IntoResponse {
    (status, Json(SuccessResponse { ok: true, data }))
}

pub fn ok<T: Serialize>(data: T) -> impl IntoResponse {
    json(StatusCode::OK, data)
}

pub fn created<T: Serialize>(data: T) -> impl IntoResponse {
    json(StatusCode::CREATED, data)
}

pub fn error(status: StatusCode, code: &'static str, message: String) -> impl IntoResponse {
    (
        status,
        Json(ErrorResponse {
            ok: false,
            error: ErrorBody { code, message },
        }),
    )
}
