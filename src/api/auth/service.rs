/// 회원가입 비즈니스 로직
///
/// Trait 기반 의존성 주입:
/// - 실제: PgUserRepository (DB 사용)
/// - 테스트: MockUserRepository (가짜 데이터)

use super::{dtos::UserResponse, repository_trait::UserRepository};
use crate::api::error::AppError;

pub async fn sign_up<R: UserRepository>(
    repo: &R,
    username: String
) -> Result<UserResponse, AppError> {
    // 1. 중복 체크
    let exists = repo.exists(&username)
        .await
        .map_err(|_| AppError::InternalServerError("Database error occurred".to_string()))?;

    if exists {
        return Err(AppError::Conflict("Username already exists".to_string()));
    }

    // 2. 저장
    let user_id = repo.save(&username)
        .await
        .map_err(|_| AppError::InternalServerError("Database error occurred".to_string()))?;

    // 3. 응답 반환
    Ok(UserResponse {
        id: user_id,
        username,
    })
}

