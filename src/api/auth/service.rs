use super::{dtos::UserResponse, error::AuthServiceError, repository_trait::UserRepository};

// ============================================================================
// 회원가입 비즈니스 로직
// ============================================================================
//
// 변경 사항:
//   Before: pool: &PgPool  → PostgreSQL에 직접 의존
//   After:  repo: &R       → UserRepository Trait에 의존
//
// 왜 바꿨나?
//   - 테스트할 때 DB 없이 Mock으로 교체 가능
//   - 나중에 MySQL로 바꿔도 이 함수는 수정 불필요
//   - "구현이 아닌 인터페이스에 의존하라" (SOLID 원칙)
//
// ============================================================================

pub async fn sign_up<R: UserRepository>(
    // 제네릭 타입 R:
    //   - R은 UserRepository를 구현한 '어떤' 타입
    //   - 실제: PgUserRepository { pool: &pool }
    //   - 테스트: MockUserRepository { should_exist: false }
    repo: &R,
    username: String
) -> Result<UserResponse, AuthServiceError> {

    // -----------------------------------------------------------------------
    // 1. 중복 체크
    // -----------------------------------------------------------------------
    // Before: repository::exist_by_username(pool, &username)
    // After:  repo.exists(&username)
    //   → Trait의 메서드 호출!
    let exists = repo.exists(&username)
        .await
        .map_err(|_| AuthServiceError::DatabaseError)?;
        // map_err: sqlx::Error를 AuthServiceError::DatabaseError로 변환
        //   - sqlx::Error는 외부 라이브러리 에러 (구체적)
        //   - AuthServiceError는 우리 비즈니스 에러 (추상적)

    if exists {
        // 사용자가 이미 존재하면 에러 반환
        return Err(AuthServiceError::UserAlreadyExists);
    }

    // -----------------------------------------------------------------------
    // 2. 저장
    // -----------------------------------------------------------------------
    // Before: repository::save_user(pool, &username)
    // After:  repo.save(&username)
    let user_id = repo.save(&username)
        .await
        .map_err(|_| AuthServiceError::DatabaseError)?;

    // -----------------------------------------------------------------------
    // 3. 응답 객체(DTO) 변환 및 반환
    // -----------------------------------------------------------------------
    Ok(UserResponse {
        id: user_id,
        username,
    })
}

// ============================================================================
// 호출 방법 비교
// ============================================================================
//
// [Before - Pool 직접 주입]
// let result = sign_up(&pool, "john".to_string()).await;
//   → DB 없으면 테스트 불가능!
//
// [After - Repository 주입]
// // 실제 환경 (Handler에서)
// let repo = PgUserRepository::new(&pool);
// let result = sign_up(&repo, "john".to_string()).await;
//
// // 테스트 환경
// let mock_repo = MockUserRepository { should_exist: false };
// let result = sign_up(&mock_repo, "john".to_string()).await;
//   → DB 없이 테스트 가능!
//
// ============================================================================

// ============================================================================
// 테스트 모듈 (Unit Tests)
// ============================================================================
// #[cfg(test)]:
//   - "test" 설정일 때만 컴파일
//   - cargo test 실행 시에만 포함됨
//   - 최종 바이너리에는 포함 안 됨 (크기 절약)
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;
    use uuid::Uuid;

    // ========================================================================
    // MockUserRepository: 테스트용 가짜 Repository
    // ========================================================================
    //
    // 목적:
    //   - DB 없이 테스트하기 위한 가짜 구현체
    //   - 원하는 대로 동작을 제어할 수 있음
    //
    // 사용 예:
    //   let mock = MockUserRepository { should_exist: true, ... };
    //   sign_up(&mock, "john").await;  // ← DB 없이 테스트!
    //
    // ========================================================================
    struct MockUserRepository {
        // ---------------------------------------------------------------
        // 테스트 시나리오 제어 필드
        // ---------------------------------------------------------------

        // exists() 호출 시 반환할 값
        //   - true: "사용자가 이미 존재함" 시뮬레이션
        //   - false: "사용자 없음" 시뮬레이션
        should_exist: bool,

        // save() 호출 시 에러 발생 여부
        //   - true: DB 에러 시뮬레이션
        //   - false: 정상 저장 시뮬레이션
        save_should_fail: bool,
    }

    // -----------------------------------------------------------------------
    // UserRepository Trait 구현 (Mock 버전)
    // -----------------------------------------------------------------------
    #[async_trait]
    impl UserRepository for MockUserRepository {
        async fn exists(&self, _username: &str) -> Result<bool, sqlx::Error> {
            // _username: 사용하지 않으므로 _ 접두사
            //   → 컴파일러 경고 방지

            // 실제 DB 조회 대신, should_exist 값 반환!
            Ok(self.should_exist)
        }

        async fn save(&self, _username: &str) -> Result<Uuid, sqlx::Error> {
            if self.save_should_fail {
                // 에러 시뮬레이션: DB에서 row를 찾지 못했다는 에러 반환
                Err(sqlx::Error::RowNotFound)
            } else {
                // 성공 시뮬레이션: 랜덤 UUID 생성해서 반환
                Ok(Uuid::new_v4())
            }
        }
    }

    // ========================================================================
    // Test Case 1: 정상 회원가입
    // ========================================================================
    //
    // 시나리오:
    //   1. 사용자가 존재하지 않음 (should_exist: false)
    //   2. 저장 성공 (save_should_fail: false)
    //   3. UserResponse 정상 반환
    //
    // ========================================================================
    #[tokio::test]  // tokio::test: async 함수를 테스트하기 위한 매크로
    async fn test_signup_success() {
        // -------------------------------------------------------------------
        // Given: 테스트 준비
        // -------------------------------------------------------------------
        // Mock Repository 생성 (사용자 없음, 저장 성공)
        let mock_repo = MockUserRepository {
            should_exist: false,      // 중복 없음
            save_should_fail: false,  // 저장 성공
        };
        let username = "john".to_string();

        // -------------------------------------------------------------------
        // When: 테스트 실행
        // -------------------------------------------------------------------
        // sign_up 호출 (DB 없이 Mock 사용!)
        let result = sign_up(&mock_repo, username).await;

        // -------------------------------------------------------------------
        // Then: 검증
        // -------------------------------------------------------------------
        // 1. 성공했는지 확인
        assert!(result.is_ok(), "회원가입이 성공해야 합니다");

        // 2. 반환된 UserResponse 확인
        let user = result.unwrap();
        assert_eq!(user.username, "john", "username이 'john'이어야 합니다");
        // user.id는 랜덤 UUID이므로 검증 생략
    }

    // ========================================================================
    // Test Case 2: 중복 사용자 에러
    // ========================================================================
    //
    // 시나리오:
    //   1. 사용자가 이미 존재 (should_exist: true)
    //   2. UserAlreadyExists 에러 반환
    //
    // ========================================================================
    #[tokio::test]
    async fn test_signup_duplicate_user() {
        // -------------------------------------------------------------------
        // Given: 이미 존재하는 사용자
        // -------------------------------------------------------------------
        let mock_repo = MockUserRepository {
            should_exist: true,       // ← 중복!
            save_should_fail: false,
        };
        let username = "john".to_string();

        // -------------------------------------------------------------------
        // When: 회원가입 시도
        // -------------------------------------------------------------------
        let result = sign_up(&mock_repo, username).await;

        // -------------------------------------------------------------------
        // Then: 에러 확인
        // -------------------------------------------------------------------
        // 1. 에러가 발생했는지
        assert!(result.is_err(), "중복 사용자는 에러를 반환해야 합니다");

        // 2. 정확히 UserAlreadyExists 에러인지
        let error = result.unwrap_err();
        assert!(
            matches!(error, AuthServiceError::UserAlreadyExists),
            "UserAlreadyExists 에러여야 합니다"
        );
        // matches!: enum variant 매칭 확인 매크로
    }

    // ========================================================================
    // Test Case 3: DB 저장 실패
    // ========================================================================
    //
    // 시나리오:
    //   1. 사용자 없음 (중복 체크 통과)
    //   2. 저장 시 DB 에러 (save_should_fail: true)
    //   3. DatabaseError 반환
    //
    // ========================================================================
    #[tokio::test]
    async fn test_signup_database_error() {
        // -------------------------------------------------------------------
        // Given: 저장 실패 상황
        // -------------------------------------------------------------------
        let mock_repo = MockUserRepository {
            should_exist: false,
            save_should_fail: true,   // ← DB 에러!
        };
        let username = "john".to_string();

        // -------------------------------------------------------------------
        // When: 회원가입 시도
        // -------------------------------------------------------------------
        let result = sign_up(&mock_repo, username).await;

        // -------------------------------------------------------------------
        // Then: DatabaseError 확인
        // -------------------------------------------------------------------
        assert!(result.is_err(), "DB 에러 시 실패해야 합니다");

        let error = result.unwrap_err();
        assert!(
            matches!(error, AuthServiceError::DatabaseError),
            "DatabaseError여야 합니다"
        );
    }

    // ========================================================================
    // Test Case 4: 빈 username
    // ========================================================================
    //
    // 시나리오:
    //   - 빈 문자열도 정상 처리되는지 확인
    //   - (실제로는 validation이 필요하지만, 지금은 없음)
    //
    // ========================================================================
    #[tokio::test]
    async fn test_signup_empty_username() {
        let mock_repo = MockUserRepository {
            should_exist: false,
            save_should_fail: false,
        };

        let result = sign_up(&mock_repo, "".to_string()).await;

        // 현재는 검증이 없어서 성공함
        // TODO: username validation 추가 시 이 테스트 수정 필요
        assert!(result.is_ok(), "현재는 빈 username도 허용됩니다");
    }
}

// ============================================================================
// 테스트 실행 방법
// ============================================================================
//
// 1. 모든 테스트 실행:
//    cargo test
//
// 2. 이 모듈의 테스트만:
//    cargo test --lib auth::service
//
// 3. 특정 테스트만:
//    cargo test test_signup_success
//
// 4. 테스트 출력 보기:
//    cargo test -- --nocapture
//
// 5. 병렬 실행 금지 (DB 테스트 시):
//    cargo test -- --test-threads=1
//
// ============================================================================

