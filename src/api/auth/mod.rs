use axum::{routing::post, Router};
use sqlx::PgPool;

// ============================================================================
// Auth 모듈 구조
// ============================================================================
//
// 하위 파일들 모듈 등록 (이게 있어야 컴파일 됨)
//
// dtos:              Request/Response 데이터 구조
// error:             Service 레이어 에러 타입
// repository_trait:  Repository 추상화 (Trait)
// repository:        실제 DB 구현체 (PgUserRepository)
// service:           비즈니스 로직 + Unit Tests
// handlers:          HTTP 요청 처리
//
// ============================================================================

mod dtos;
mod error;
mod repository_trait;  // ← 새로 추가: Trait 정의
mod repository;
mod service;
mod handlers;

pub fn router() -> Router<PgPool> {
    Router::new()
        .route("/signup", post(handlers::signup_handler))
        // .route("/login", ...)
}