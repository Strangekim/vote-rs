/// Service 레이어 Unit Tests
///
/// DB 없이 Mock Repository를 사용하여 테스트
/// 실행: cargo test

use super::service::sign_up;
use super::error::AuthServiceError;
use super::repository_trait::UserRepository;
use async_trait::async_trait;
use uuid::Uuid;

/// 테스트용 Mock Repository
struct MockUserRepository {
    should_exist: bool,     // 중복 사용자 시뮬레이션
    save_should_fail: bool, // DB 에러 시뮬레이션
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

#[tokio::test]
async fn test_signup_success() {
    let mock_repo = MockUserRepository {
        should_exist: false,
        save_should_fail: false,
    };

    let result = sign_up(&mock_repo, "john".to_string()).await;

    assert!(result.is_ok());
    assert_eq!(result.unwrap().username, "john");
}

#[tokio::test]
async fn test_signup_duplicate_user() {
    let mock_repo = MockUserRepository {
        should_exist: true,
        save_should_fail: false,
    };

    let result = sign_up(&mock_repo, "john".to_string()).await;

    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), AuthServiceError::UserAlreadyExists));
}

#[tokio::test]
async fn test_signup_database_error() {
    let mock_repo = MockUserRepository {
        should_exist: false,
        save_should_fail: true,
    };

    let result = sign_up(&mock_repo, "john".to_string()).await;

    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), AuthServiceError::DatabaseError));
}

#[tokio::test]
async fn test_signup_empty_username() {
    let mock_repo = MockUserRepository {
        should_exist: false,
        save_should_fail: false,
    };

    let result = sign_up(&mock_repo, "".to_string()).await;

    // TODO: validation 추가 시 수정 필요
    assert!(result.is_ok());
}
