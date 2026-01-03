use axum::{routing::post, Router};
use sqlx::PgPool;

// 하위 파일들 모듈 등록 (이게 있어야 컴파일 됨)
mod dtos;
mod repository;
mod service;
mod handlers;
mod error;

pub fn router() -> Router<PgPool> {
    Router::new()
        .route("/signup", post(handlers::signup_handler))
        // .route("/login", ...)
}