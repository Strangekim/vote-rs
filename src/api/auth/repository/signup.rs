use sqlx::PgPool;
use uuid::Uuid;

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
