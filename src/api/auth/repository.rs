use sqlx::PgPool;
use uuid::Uuid;

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