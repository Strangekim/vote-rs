/// 회원가입 비즈니스 로직
///
/// Trait 기반 의존성 주입:
/// - 실제: PgUserRepository (DB 사용)
/// - 테스트: MockUserRepository (가짜 데이터)

use super::{dtos::{UserResponse, LoginResponse}, repository::traits::UserRepository};
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

/// 로그인 비즈니스 로직
///
/// 1. DB에서 사용자 조회
/// 2. 사용자가 없으면 Unauthorized 에러 반환
/// 3. 사용자가 있으면 성공 응답 반환 (추후 비밀번호 체크 추가 필요)
use uuid::Uuid; // Added for token generation

pub async fn login<R: UserRepository>(
    repo: &R,
    username: String
) -> Result<LoginResponse, AppError> {
    // 1. 사용자 조회
    let user = repo.find_by_username(&username)
        .await
        .map_err(|_| AppError::InternalServerError("Database error occurred".to_string()))?;

    // 2. 결과 처리
    match user {
        Some(u) => {
            // JWT 토큰 발급
            let token = super::jwt::generate_token(u.id, u.username.clone())?;
            
            Ok(LoginResponse { 
                token, 
                user_id: u.id, 
                username: u.username 
            })
        },
        None => Err(AppError::Unauthorized("Invalid username".to_string())),
    }
}

