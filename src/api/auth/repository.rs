use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;
use super::repository_trait::UserRepository;

#[derive(sqlx::FromRow)]
struct UserEntity {
    id: Uuid,
    username: String,
}

/// 사용자 존재 여부 확인
pub async fn exist_by_username(pool: &PgPool, username: &str) -> Result<bool, sqlx::Error> {
    let result = sqlx::query!(
        "SELECT count(*) as count FROM users WHERE username = $1",
        username
    )
    .fetch_one(pool)
    .await?;

    Ok(result.count.unwrap_or(0) > 0)
}

/// 사용자 생성
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
        exist_by_username(self.pool, username).await
    }

    async fn save(&self, username: &str) -> Result<Uuid, sqlx::Error> {
        save_user(self.pool, username).await
    }
}
