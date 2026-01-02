use sqlx::{postgres::PgPoolOptions, PgPool};
use std::env;

// DB 연결 초기화 함수
// pub : 다른 모듈에서 불러다 쓸 수 있게 공개
// 반환 값은 연결이 완료된 PgPool 객체

pub async fn init_pool() -> PgPool {
    // 1. 환경 변수 읽기
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL 이 존재하지 않습니다.");

    // 2. 연결 풀 생성 설정
    println!("⏳ Connecting to Database...");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("PostgreSQL 연결 실패");

    println!("✅ Connection to the database is successful!");

    pool // 완성된 pool 반환
}