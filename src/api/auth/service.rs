use sqlx::PgPool;
use super::{dtos::UserResponse, repository}; // repository, dto 가져오기

// 회원가입 비즈니스 로직
pub async fn sign_up(pool: &PgPool, username: String) -> Result<UserResponse, String> {
    // 1. 중복 체크 (Repository 호출)
    let exists = repository::exist_by_username(pool, &username)
        .await
        .map_err(|_| "DB Error")?; // 에러 처리 간소화

    if exists {
        return Err("Username already exists".to_string());
    }

    // 2. 저장 (Repository 호출)
    let user_id = repository::save_user(pool, username.clone())
        .await
        .map_err(|_| "DB Save Error")?;

    // 3. 응답 객체(DTO) 변환 및 반환
    Ok(UserResponse {
        id: user_id,
        username,
    })
}