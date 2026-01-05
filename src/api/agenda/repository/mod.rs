use serde::Serialize;
use uuid::Uuid;
use chrono::{DateTime, Utc};

pub mod traits;
pub mod create;

// Agenda Entity
#[derive(Debug, Clone, Serialize)]
pub struct AgendaEntity {
    pub id: Uuid,
    pub title: String,
    // description removed
    pub created_by: Uuid,
    pub created_at: DateTime<Utc>,
    pub agree_count: i32,
    pub disagree_count: i32,
}

pub struct PgAgendaRepository {
    pool: sqlx::PgPool,
}

impl PgAgendaRepository {
    pub fn new(pool: &sqlx::PgPool) -> Self {
        Self { pool: pool.clone() }
    }
}

use async_trait::async_trait;
use self::traits::AgendaRepository;

#[async_trait]
impl AgendaRepository for PgAgendaRepository {
    async fn create(&self, title: &str, created_by: Uuid) -> Result<AgendaEntity, sqlx::Error> {
        self::create::create(&self.pool, title, created_by).await
    }
}
