use crate::api::auth::service::login;
use crate::api::auth::repository::UserEntity;
use crate::api::error::AppError;
use super::MockUserRepository;
use uuid::Uuid;

// 테스트 1: 로그인 성공
// 올바른 username이 주어졌을 때, 해당 사용자 정보를 반환해야 함
#[tokio::test]
async fn test_login_success() {
    // Mock 데이터 준비: DB에 "john" 유저가 있다고 가정
    let user = UserEntity {
        id: Uuid::new_v4(),
        username: "john".to_string(),
    };

    let mock_repo = MockUserRepository {
        find_result: Some(user.clone()), // 조회 시 유저 반환
        ..Default::default()
    };

    // Service 호출
    let result = login(&mock_repo, "john".to_string()).await;
    
    // 검증
    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response.username, "john");
    assert!(!response.token.is_empty()); // 토큰 존재 확인
    assert_eq!(response.user_id, user.id);
}

// 테스트 2: 로그인 실패 (존재하지 않는 사용자)
// DB에 없는 username으로 요청 시, Unauthorized 에러 반환해야 함
#[tokio::test]
async fn test_login_not_found() {
    let mock_repo = MockUserRepository {
        find_result: None, // 조회 결과 없음
        ..Default::default()
    };

    let result = login(&mock_repo, "unknown".to_string()).await;
    
    // 검증: Unauthorized 에러 발생하는지 확인
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), AppError::Unauthorized(_)));
}
