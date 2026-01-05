use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;
use self::traits::UserRepository;

pub mod traits;
pub mod signup;
pub mod login; // New module

#[derive(sqlx::FromRow, Debug, Clone)] // Added Debug, Clone
pub struct UserEntity {
    pub id: Uuid,
    pub username: String,
}

/// PostgreSQL Repository 구현체
/// Handler에서 실제 DB 작업에 사용
pub struct PgUserRepository<'a> {
    pool: &'a PgPool,
}

impl<'a> PgUserRepository<'a> {
    pub fn new(pool: &'a PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl<'a> UserRepository for PgUserRepository<'a> {
    async fn exists(&self, username: &str) -> Result<bool, sqlx::Error> {
        signup::exist_by_username(self.pool, username).await
    }

    async fn find_by_username(&self, username: &str) -> Result<Option<UserEntity>, sqlx::Error> {
        login::find_by_username(self.pool, username).await
    }

    async fn save(&self, username: &str) -> Result<Uuid, sqlx::Error> {
        signup::save_user(self.pool, username).await
    }
}
