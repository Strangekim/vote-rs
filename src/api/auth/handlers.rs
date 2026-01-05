use axum::{extract::State, Json, http::StatusCode};
use sqlx::PgPool;
use super::{dtos::{SignupRequest, LoginRequest}, service, repository::PgUserRepository};
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

/// 로그인 핸들러
///
/// 흐름: HTTP Request (username) → Service 호출 (find) → 성공 시 사용자 정보 반환
/// - 성공: 200 OK + UserResponse
/// - 실패: 401 Unauthorized (Service에서 에러 발생)
pub async fn login_handler(
    State(pool): State<PgPool>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<super::dtos::LoginResponse>, AppError> { // Updated return type
    let repo = PgUserRepository::new(&pool);
    let user_res = service::login(&repo, payload.username).await?;
    Ok(Json(user_res))
}
