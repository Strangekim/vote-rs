use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct CreateAgendaRequest {
    pub title: String,
}

#[derive(Serialize)]
pub struct AgendaResponse {
    pub id: uuid::Uuid,
    pub title: String,
    pub created_by: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub agree_count: i32,
    pub disagree_count: i32,
}
