use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::api::error::AppError;
use std::time::{SystemTime, UNIX_EPOCH};

const DEFAULT_SECRET: &str = "secret";

/// JWT Payload 구조체
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,    // User ID
    pub username: String,
    pub exp: usize,     // Expiration Time
    pub iat: usize,     // Issued At
}

/// JWT 토큰 생성
///
/// - `sub`: user_id
/// - `exp`: 현재시간 + 24시간
pub fn generate_token(user_id: Uuid, username: String) -> Result<String, AppError> {
    let expiration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as usize + 24 * 3600; // 24 hours

    let claims = Claims {
        sub: user_id.to_string(),
        username,
        exp: expiration,
        iat: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as usize,
    };

    // 환경 변수에서 SECRET 가져오기 (없으면 기본값 사용 - 개발용)
    // 실제 운영 배포 시에는 반드시 JWT_SECRET을 설정해야 함!
    let secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| DEFAULT_SECRET.to_string());

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .map_err(|_| AppError::InternalServerError("Token generation failed".to_string()))
}

/// JWT 토큰 검증
pub fn verify_token(token: &str) -> Result<Claims, AppError> {
    let secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| DEFAULT_SECRET.to_string());
    
    let validation = jsonwebtoken::Validation::new(jsonwebtoken::Algorithm::HS256);
    
    let token_data = jsonwebtoken::decode::<Claims>(
        token,
        &jsonwebtoken::DecodingKey::from_secret(secret.as_bytes()),
        &validation,
    )
    .map_err(|_| AppError::Unauthorized("Invalid token".to_string()))?;

    Ok(token_data.claims)
}

use axum::{
    async_trait,
    extract::FromRequestParts,
    http::request::Parts,
};

/// Axum Extractor for Claims
///
/// 핸들러에서 `claims: Claims` 형태로 사용하면 자동으로 헤더에서 토큰을 추출하고 검증합니다.
#[async_trait]
impl<S> FromRequestParts<S> for Claims
where
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // 1. Authorization 헤더 가져오기
        let auth_header = parts
            .headers
            .get(axum::http::header::AUTHORIZATION)
            .ok_or(AppError::Unauthorized("Missing Authorization header".to_string()))?;

        // 2. Bearer 토큰 파싱
        let auth_str = auth_header
            .to_str()
            .map_err(|_| AppError::Unauthorized("Invalid Authorization header".to_string()))?;

        if !auth_str.starts_with("Bearer ") {
            return Err(AppError::Unauthorized("Invalid bearer token format".to_string()));
        }

        let token = &auth_str[7..];

        // 3. 토큰 검증
        verify_token(token)
    }
}
