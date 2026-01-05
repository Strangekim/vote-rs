use uuid::Uuid;
use crate::api::error::AppError;
use super::repository::traits::AgendaRepository;
use super::dtos::AgendaResponse;

/// 안건 생성 비즈니스 로직
pub async fn create_agenda<R: AgendaRepository>(
    repo: &R,
    title: String,
    created_by: Uuid // Renamed for consistency
) -> Result<AgendaResponse, AppError> {
    
    // DB 저장
    let agenda = repo.create(&title, created_by)
        .await
        .map_err(|e| AppError::InternalServerError(format!("Database error: {}", e)))?;

    // 응답 변환
    Ok(AgendaResponse {
        id: agenda.id,
        title: agenda.title,
        created_by: agenda.created_by.to_string(), // UUID -> String
        created_at: agenda.created_at,
        agree_count: 0,
        disagree_count: 0,
    })
}
