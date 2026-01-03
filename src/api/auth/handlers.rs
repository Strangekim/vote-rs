use axum::{extract::State, Json, http::StatusCode};
use sqlx::PgPool;
use super::{dtos::SignupRequest, service, error::AuthServiceError, repository::PgUserRepository};
use crate::api::error::AppError;

// ============================================================================
// 회원가입 핸들러 (HTTP 요청 처리)
// ============================================================================
//
// 역할:
//   1. HTTP 요청 받기 (Extractor로 자동 파싱)
//   2. Repository 생성 (실제 DB 사용)
//   3. Service 호출 (비즈니스 로직)
//   4. 에러 변환 (AuthServiceError → AppError)
//   5. HTTP 응답 반환
//
// ============================================================================

pub async fn signup_handler(
    // -----------------------------------------------------------------------
    // Axum Extractors (자동 주입)
    // -----------------------------------------------------------------------
    State(pool): State<PgPool>,              // 앱 상태에서 DB pool 가져오기
    Json(payload): Json<SignupRequest>,      // HTTP body를 SignupRequest로 파싱
) -> Result<(StatusCode, Json<super::dtos::UserResponse>), AppError> {

    // -----------------------------------------------------------------------
    // 1. Repository 생성 (실제 DB 구현체)
    // -----------------------------------------------------------------------
    // Before: service::sign_up(&pool, ...)
    //   → Service가 pool에 직접 의존
    //
    // After: Repository로 감싸서 전달
    //   → Service는 Trait에만 의존
    //   → 테스트 시 Mock으로 교체 가능
    let repo = PgUserRepository::new(&pool);

    // -----------------------------------------------------------------------
    // 2. Service 호출 (비즈니스 로직 실행)
    // -----------------------------------------------------------------------
    // service::sign_up()은 제네릭 함수:
    //   - 여기서는 PgUserRepository를 받음
    //   - 테스트에서는 MockUserRepository를 받음
    let user_res = service::sign_up(&repo, payload.username).await
        .map_err(|err| match err {
            // -----------------------------------------------------------
            // 3. 에러 변환 (Service 에러 → HTTP 에러)
            // -----------------------------------------------------------
            // AuthServiceError (비즈니스 레이어)
            //   → AppError (HTTP 레이어)
            //   → JSON 응답 (최종)

            AuthServiceError::UserAlreadyExists =>
                // 409 Conflict + { "message": "Username already exists" }
                AppError::Conflict("Username already exists".to_string()),

            AuthServiceError::DatabaseError =>
                // 500 Internal Server Error + { "message": "Database error occurred" }
                AppError::InternalServerError("Database error occurred".to_string()),
        })?;

    // -----------------------------------------------------------------------
    // 4. 성공 응답 반환
    // -----------------------------------------------------------------------
    // HTTP 201 Created + JSON body
    Ok((StatusCode::CREATED, Json(user_res)))
}

// ============================================================================
// 레이어 구조 정리
// ============================================================================
//
// [HTTP Request]
//      ↓
// ┌──────────────────────────────────┐
// │ Handler (handlers.rs)            │  ← HTTP 레이어
// │  - 요청 파싱                     │
// │  - Repository 생성               │
// │  - 에러 변환                     │
// └────────────┬─────────────────────┘
//              ↓ service::sign_up(&repo, username)
// ┌──────────────────────────────────┐
// │ Service (service.rs)             │  ← 비즈니스 로직 레이어
// │  - 비즈니스 규칙 검증            │
// │  - Repository 호출               │
// │  - AuthServiceError 반환         │
// └────────────┬─────────────────────┘
//              ↓ repo.exists() / repo.save()
// ┌──────────────────────────────────┐
// │ Repository (repository.rs)       │  ← 데이터 접근 레이어
// │  - SQL 쿼리 실행                 │
// │  - DB 에러 처리                  │
// │  - sqlx::Error 반환              │
// └────────────┬─────────────────────┘
//              ↓
//        [PostgreSQL DB]
//
// ============================================================================
