use axum::{Json, extract::State};
use sqlx::PgPool;
use crate::api::auth::jwt::Claims;
use crate::api::error::AppError;
use super::dtos::{CreateAgendaRequest, AgendaResponse};
use super::repository::PgAgendaRepository;
use super::service;

/// 안건 생성 핸들러
///
/// - `claims`: JWT 토큰에서 추출한 사용자 정보 (인증 필수)
pub async fn create_agenda(
    State(pool): State<PgPool>,
    claims: Claims, // JWT 인증 (Authorization header required)
    Json(payload): Json<CreateAgendaRequest>,
) -> Result<Json<AgendaResponse>, AppError> {
    let repo = PgAgendaRepository::new(&pool);
    
    // Claims의 sub(subject)는 user_id (String) 이므로 Uuid로 파싱 필요
    let user_id = uuid::Uuid::parse_str(&claims.sub)
        .map_err(|_| AppError::Unauthorized("Invalid token user_id".to_string()))?;

    let response = service::create_agenda(&repo, payload.title, user_id).await?;
    
    Ok(Json(response))
}
