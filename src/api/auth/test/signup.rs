use crate::api::auth::service::sign_up;
use crate::api::error::AppError;
use super::MockUserRepository;

// 테스트 1: 정상 회원가입
#[tokio::test]
async fn test_signup_success() {
    let mock_repo = MockUserRepository {
        should_exist: false,
        save_should_fail: false,
        ..Default::default()
    };

    let result = sign_up(&mock_repo, "john".to_string()).await;

    assert!(result.is_ok());
    assert_eq!(result.unwrap().username, "john");
}

// 테스트 2: 중복 사용자 에러
#[tokio::test]
async fn test_signup_duplicate_user() {
    let mock_repo = MockUserRepository {
        should_exist: true,
        save_should_fail: false,
        ..Default::default()
    };

    let result = sign_up(&mock_repo, "john".to_string()).await;

    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), AppError::Conflict(_)));
}

// 테스트 3: DB 저장 실패
#[tokio::test]
async fn test_signup_database_error() {
    let mock_repo = MockUserRepository {
        save_should_fail: true,
        ..Default::default()
    };

    let result = sign_up(&mock_repo, "john".to_string()).await;

    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), AppError::InternalServerError(_)));
}

// 테스트 4: 빈 username (엣지 케이스)
#[tokio::test]
async fn test_signup_empty_username() {
    let mock_repo = MockUserRepository::default();

    let result = sign_up(&mock_repo, "".to_string()).await;

    // TODO: validation 추가 시 실패하도록 수정
    assert!(result.is_ok());
}
