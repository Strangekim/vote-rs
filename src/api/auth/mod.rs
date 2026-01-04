use axum::{routing::post, Router};
use sqlx::PgPool;

// Auth 모듈 구조
// - dtos: Request/Response 데이터
// - error: Service 레이어 에러
// - repository_trait: Repository 추상화 (Trait)
// - repository: DB 구현체 (PgUserRepository)
// - service: 비즈니스 로직 + Unit Tests
// - handlers: HTTP 요청 처리

mod dtos;
mod error;
mod repository_trait;
mod repository;
pub mod service;  // pub으로 변경 (테스트에서 접근 가능하도록)
mod handlers;

#[cfg(test)]
mod service_test;  // 테스트 모듈 등록

pub fn router() -> Router<PgPool> {
    Router::new()
        .route("/signup", post(handlers::signup_handler))
        // .route("/login", ...)
}