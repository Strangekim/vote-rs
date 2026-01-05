use serde::{Deserialize, Serialize};
use uuid::Uuid;

// [Request] 회원가입 요청 데이터
#[derive(Deserialize)]
pub struct SignupRequest {
    pub username: String,
}

// [Request] 로그인 요청 데이터
#[derive(Deserialize)]
pub struct LoginRequest {
    pub username: String,
}

// [Response] 응답 데이터 (회원가입)
#[derive(Serialize, Debug)]
pub struct UserResponse {
    pub id: Uuid,
    pub username: String,
}

// [Response] 응답 데이터 (로그인)
#[derive(Serialize, Debug)]
pub struct LoginResponse {
    pub token: String,
    pub user_id: Uuid,
    pub username: String,
}