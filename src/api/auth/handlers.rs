use axum::{extract::State, Json, response::IntoResponse, http::StatusCode};
use sqlx::PgPool;
use super::{dtos::SignupRequest, service}; // service, dto 가져오기

pub async fn signup_handler(
    State(pool): State<PgPool>,
    Json(payload): Json<SignupRequest>, // DTO로 바디 파싱
) -> impl IntoResponse {
    
    // 서비스 호출
    match service::sign_up(&pool, payload.username).await {
        Ok(user_res) => {
            // 성공 시 201 Created와 JSON 반환
            (StatusCode::CREATED, Json(user_res)).into_response()
        },
        Err(err_msg) => {
            // 실패 시 에러 메시지 반환 (실무에선 에러 코드 분기 필요)
            if err_msg == "Username already exists" {
                (StatusCode::CONFLICT, err_msg).into_response()
            } else {
                (StatusCode::INTERNAL_SERVER_ERROR, "Server Error").into_response()
            }
        }
    }
}