use sqlx::PgPool;
use super::{dtos::UserResponse, repository, error::AuthServiceError};

// 회원가입 비즈니스 로직
pub async fn sign_up(pool: &PgPool, username: String) -> Result<UserResponse, AuthServiceError> {
    // 1. 중복 체크 (Repository 호출)
    let exists = repository::exist_by_username(pool, &username)
        .await
        .map_err(|_| AuthServiceError::DatabaseError)?;

    if exists {
        return Err(AuthServiceError::UserAlreadyExists);
    }

    // 2. 저장 (Repository 호출)
    let user_id = repository::save_user(pool, &username)
        .await
        .map_err(|_| AuthServiceError::DatabaseError)?;

    // 3. 응답 객체(DTO) 변환 및 반환
    Ok(UserResponse {
        id: user_id,
        username,
    })
}