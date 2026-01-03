use axum::{extract::State, Json, http::StatusCode};
use sqlx::PgPool;
use super::{dtos::SignupRequest, service, error::AuthServiceError};
use crate::api::error::AppError;

pub async fn signup_handler(
    State(pool): State<PgPool>,
    Json(payload): Json<SignupRequest>,
) -> Result<(StatusCode, Json<super::dtos::UserResponse>), AppError> {
    // 서비스 호출
    let user_res = service::sign_up(&pool, payload.username).await
        .map_err(|err| match err {
            // 에러 타입에 따라 적절한 상태 코드 + 메시지 매핑
            AuthServiceError::UserAlreadyExists =>
                AppError::Conflict("Username already exists".to_string()),
            AuthServiceError::DatabaseError =>
                AppError::InternalServerError("Database error occurred".to_string()),
        })?;

    // 성공 시 201 Created와 JSON 반환
    Ok((StatusCode::CREATED, Json(user_res)))
}