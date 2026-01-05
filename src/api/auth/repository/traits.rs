/// Repository 추상화 Trait
///
/// Service 레이어가 구체적인 DB 구현에 의존하지 않도록 함
/// - 실제: PgUserRepository (PostgreSQL)
/// - 테스트: MockUserRepository (가짜 데이터)

use async_trait::async_trait;
use uuid::Uuid;

use super::UserEntity;

#[async_trait]
pub trait UserRepository: Send + Sync {
    /// 사용자 존재 여부 확인
    async fn exists(&self, username: &str) -> Result<bool, sqlx::Error>;

    /// 사용자 조회 (로그인용)
    async fn find_by_username(&self, username: &str) -> Result<Option<UserEntity>, sqlx::Error>;

    /// 사용자 저장 후 생성된 ID 반환
    async fn save(&self, username: &str) -> Result<Uuid, sqlx::Error>;
}
