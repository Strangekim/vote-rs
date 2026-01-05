use sqlx::PgPool;
use uuid::Uuid;
use super::AgendaEntity;

/// 안건 생성 DB 로직
pub async fn create(pool: &PgPool, title: &str, created_by: Uuid) -> Result<AgendaEntity, sqlx::Error> {
    sqlx::query_as!(
        AgendaEntity,
        "INSERT INTO agendas (id, title, created_by, created_at, agree_count, disagree_count) 
         VALUES ($1, $2, $3, NOW(), 0, 0) 
         RETURNING id, title, created_by, created_at, agree_count, disagree_count",
        Uuid::new_v4(),
        title,
        created_by
    )
    .fetch_one(pool)
    .await
}
