use async_trait::async_trait;
use uuid::Uuid;
use super::AgendaEntity;

#[async_trait]
pub trait AgendaRepository: Send + Sync {
    /// 안건 생성
    async fn create(&self, title: &str, created_by: Uuid) -> Result<AgendaEntity, sqlx::Error>;
}
