use serde::{Deserialize, Serialize};
use uuid::Uuid;

// [Request] 회원가입 요청 데이터
#[derive(Deserialize)]
pub struct SignupRequest {
    pub username: String,
}

// [Response] 응답 데이터
#[derive(Serialize)]
pub struct UserResponse {
    pub id: Uuid,
    pub username: String,
}