use super::repository::traits::AgendaRepository;
use super::repository::AgendaEntity;
use async_trait::async_trait;
use uuid::Uuid;

pub mod create;

pub struct MockAgendaRepository {
    pub should_fail: bool,
}

impl Default for MockAgendaRepository {
    fn default() -> Self {
        Self { should_fail: false }
    }
}

#[async_trait]
impl AgendaRepository for MockAgendaRepository {
    async fn create(&self, title: &str, created_by: Uuid) -> Result<AgendaEntity, sqlx::Error> {
        if self.should_fail {
            Err(sqlx::Error::RowNotFound)
        } else {
            Ok(AgendaEntity {
                id: Uuid::new_v4(),
                title: title.to_string(),
                // description removed
                created_by,
                created_at: chrono::Utc::now(),
                agree_count: 0,
                disagree_count: 0,
            })
        }
    }
}
