use axum::{response::IntoResponse, http::StatusCode, Json};
use serde::Serialize;

// 공통 에러 응답 DTO
#[derive(Serialize)]
pub struct ErrorResponse {
    pub message: String,
}

// 커스텀 에러 타입
pub enum AppError {
    NotFound(String),
    Conflict(String),
    BadRequest(String),
    InternalServerError(String),
}

// IntoResponse 구현 - 자동으로 HTTP 응답으로 변환
impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let (status, message) = match self {
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            AppError::Conflict(msg) => (StatusCode::CONFLICT, msg),
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
            AppError::InternalServerError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
        };

        let body = ErrorResponse { message };

        (status, Json(body)).into_response()
    }
}
