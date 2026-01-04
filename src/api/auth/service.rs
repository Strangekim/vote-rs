/// 회원가입 비즈니스 로직
///
/// Trait 기반 의존성 주입:
/// - 실제: PgUserRepository (DB 사용)
/// - 테스트: MockUserRepository (가짜 데이터)

use super::{dtos::UserResponse, error::AuthServiceError, repository_trait::UserRepository};

pub async fn sign_up<R: UserRepository>(
    repo: &R,
    username: String
) -> Result<UserResponse, AuthServiceError> {
    // 1. 중복 체크
    let exists = repo.exists(&username)
        .await
        .map_err(|_| AuthServiceError::DatabaseError)?;

    if exists {
        return Err(AuthServiceError::UserAlreadyExists);
    }

    // 2. 저장
    let user_id = repo.save(&username)
        .await
        .map_err(|_| AuthServiceError::DatabaseError)?;

    // 3. 응답 반환
    Ok(UserResponse {
        id: user_id,
        username,
    })
}

