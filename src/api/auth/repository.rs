use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;
use super::repository_trait::UserRepository;

// DB 엔티티 구조체 (DTO와 다를 수 있음)
#[derive(sqlx::FromRow)] 
struct UserEntity {
    id: Uuid,
    username: String,
}

// 1. 유저 존재 여부 확인 (SELECT)
pub async fn exist_by_username(pool: &PgPool, username: &str) -> Result<bool, sqlx::Error> {
    // query! 매크로는 여기서 사용! 컴파일 타임 검증됨.
    let result = sqlx::query!(
        "SELECT count(*) as count FROM users WHERE username = $1",
        username
    )
    .fetch_one(pool)
    .await?;

    // count가 있으면 0보다 큰지 확인
    Ok(result.count.unwrap_or(0) > 0)
}

// 2. 유저 생성 (INSERT)
pub async fn save_user(pool: &PgPool, username: &str) -> Result<Uuid, sqlx::Error> {
    let new_id = Uuid::new_v4();
    
    sqlx::query!(
        "INSERT INTO users (id, username, created_at) VALUES ($1, $2, NOW())",
        new_id,
        username
    )
    .execute(pool)
    .await?;

    Ok(new_id)
}

// ============================================================================
// PgUserRepository: PostgreSQL을 사용하는 실제 구현체
// ============================================================================
//
// 목적:
//   - UserRepository Trait의 실제 구현
//   - 실제 PostgreSQL DB와 통신
//   - Handler에서 사용 (실제 운영 환경)
//
// 사용 예:
//   let repo = PgUserRepository::new(&pool);
//   service::sign_up(&repo, username).await;
//
// ============================================================================

pub struct PgUserRepository<'a> {
    // 'a: 라이프타임 파라미터
    //   - pool의 수명이 PgUserRepository보다 길어야 함
    //   - 즉, pool이 먼저 drop되면 안 됨
    pool: &'a PgPool,
}

impl<'a> PgUserRepository<'a> {
    // -----------------------------------------------------------------------
    // 생성자: PgPool 참조를 받아서 Repository 생성
    // -----------------------------------------------------------------------
    pub fn new(pool: &'a PgPool) -> Self {
        Self { pool }
    }
}

// -----------------------------------------------------------------------
// UserRepository Trait 구현
// -----------------------------------------------------------------------
// 이렇게 하면 PgUserRepository가 UserRepository로 사용 가능!
// Service는 "어떤 Repository인지" 모르고, Trait만 알면 됨
// -----------------------------------------------------------------------
#[async_trait]
impl<'a> UserRepository for PgUserRepository<'a> {
    // UserRepository::exists() 구현
    // 실제로는 위에 정의한 exist_by_username() 함수 호출
    async fn exists(&self, username: &str) -> Result<bool, sqlx::Error> {
        exist_by_username(self.pool, username).await
    }

    // UserRepository::save() 구현
    // 실제로는 위에 정의한 save_user() 함수 호출
    async fn save(&self, username: &str) -> Result<Uuid, sqlx::Error> {
        save_user(self.pool, username).await
    }
}

// ============================================================================
// 구조 정리
// ============================================================================
//
// [기존 함수들] - 실제 DB 작업 수행
//   ├─ exist_by_username(pool, username) → SELECT 쿼리 실행
//   └─ save_user(pool, username) → INSERT 쿼리 실행
//
// [새로 추가된 구조체] - Trait 구현 (어댑터 패턴)
//   └─ PgUserRepository
//        ├─ new(pool) → 생성자
//        ├─ exists() → exist_by_username() 호출
//        └─ save() → save_user() 호출
//
// 왜 이렇게?
//   - 기존 코드 유지하면서 Trait 도입
//   - 점진적 마이그레이션 가능
//
// ============================================================================
