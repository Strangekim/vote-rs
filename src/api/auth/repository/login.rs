use sqlx::PgPool;
use super::UserEntity;

/// 사용자 조회 (로그인용)
///
/// 주어진 `username` 이 일치하는 사용자를 DB에서 조회합니다.
/// - `Some(UserEntity)`: 사용자 존재 (로그인 성공 가능성 있음)
/// - `None`: 사용자 없음 (로그인 실패)
pub async fn find_by_username(pool: &PgPool, username: &str) -> Result<Option<UserEntity>, sqlx::Error> {
    sqlx::query_as!(
        UserEntity,
        "SELECT id, username FROM users WHERE username = $1",
        username
    )
    .fetch_optional(pool)
    .await
}
