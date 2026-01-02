use axum::{Router, routing::get};
use sqlx::PgPool;

// 하위 모듈(파일)들을 등록합니다.
// 이렇게 선언해야 auth.rs, agenda.rs를 인식합니다.
mod auth;
mod agenda;

// Express의 app.use('/path', router)와 같은 역할을 하는 함수를 만듭니다.
// 외부(main.rs)에서 이 함수를 호출해서 완성된 Router를 받아갑니다.
pub fn app(pool: PgPool) -> Router {
    Router::new()
        .route("/health", get(health_check)) // 공통 헬스 체크
        .nest("/auth", auth::router())       // 1. Auth 라우터 연결
        .nest("/agendas", agenda::router())  // 2. Agenda 라우터 연결
        .with_state(pool)                    // 3. DB Pool 공유 (최상단에서 한 번만 주입)
}

async fn health_check() -> &'static str {
    "I'm alive!"
}