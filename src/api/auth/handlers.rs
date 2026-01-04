use axum::{extract::State, Json, http::StatusCode};
use sqlx::PgPool;
use super::{dtos::SignupRequest, service, repository::PgUserRepository};
use crate::api::error::AppError;

/// 회원가입 핸들러
///
/// 흐름: HTTP Request → Repository 생성 → Service 호출 → HTTP Response
pub async fn signup_handler(
    State(pool): State<PgPool>,
    Json(payload): Json<SignupRequest>,
) -> Result<(StatusCode, Json<super::dtos::UserResponse>), AppError> {
    // 1. Repository 생성 (실제 DB)
    let repo = PgUserRepository::new(&pool);

    // 2. Service 호출 (에러는 이미 AppError로 반환됨)
    let user_res = service::sign_up(&repo, payload.username).await?;

    // 3. 성공 응답
    Ok((StatusCode::CREATED, Json(user_res)))
}
