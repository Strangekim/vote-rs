/// Service 레이어 Unit Tests - DB 없이 Mock으로 테스트

use super::service::sign_up;
use super::error::AuthServiceError;
use super::repository_trait::UserRepository;
use async_trait::async_trait;
use uuid::Uuid;

/// Mock Repository: DB 대신 가짜 데이터 반환
struct MockUserRepository {
    should_exist: bool,     // true면 중복 사용자
    save_should_fail: bool, // true면 DB 저장 실패
}

#[async_trait]
impl UserRepository for MockUserRepository {
    async fn exists(&self, _username: &str) -> Result<bool, sqlx::Error> {
        Ok(self.should_exist)
    }

    async fn save(&self, _username: &str) -> Result<Uuid, sqlx::Error> {
        if self.save_should_fail {
            Err(sqlx::Error::RowNotFound)
        } else {
            Ok(Uuid::new_v4())
        }
    }
}

// 테스트 1: 정상 회원가입
#[tokio::test]
async fn test_signup_success() {
    let mock_repo = MockUserRepository {
        should_exist: false,     // 중복 없음
        save_should_fail: false, // 저장 성공
    };

    let result = sign_up(&mock_repo, "john".to_string()).await;

    assert!(result.is_ok());
    assert_eq!(result.unwrap().username, "john");
}

// 테스트 2: 중복 사용자 에러
#[tokio::test]
async fn test_signup_duplicate_user() {
    let mock_repo = MockUserRepository {
        should_exist: true,      // 이미 존재
        save_should_fail: false,
    };

    let result = sign_up(&mock_repo, "john".to_string()).await;

    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), AuthServiceError::UserAlreadyExists));
}

// 테스트 3: DB 저장 실패
#[tokio::test]
async fn test_signup_database_error() {
    let mock_repo = MockUserRepository {
        should_exist: false,
        save_should_fail: true, // DB 에러
    };

    let result = sign_up(&mock_repo, "john".to_string()).await;

    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), AuthServiceError::DatabaseError));
}

// 테스트 4: 빈 username (엣지 케이스)
#[tokio::test]
async fn test_signup_empty_username() {
    let mock_repo = MockUserRepository {
        should_exist: false,
        save_should_fail: false,
    };

    let result = sign_up(&mock_repo, "".to_string()).await;

    // TODO: validation 추가 시 실패하도록 수정
    assert!(result.is_ok());
}
