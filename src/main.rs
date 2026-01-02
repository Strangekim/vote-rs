use axum::serve;
use dotenvy::dotenv;
use tokio::net::TcpListener;

// [모듈 등록]
// 파일 시스템의 src/db.rs를 찾아서 'db'라는 이름의 모듈로 인식합니다.
mod db; 
mod api;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok(); // 환경변수 로드

    // 1. DB 연결 (db 모듈에게 위임)
    // Node.js: const pool = await require('./db').initPool();
    let pool = db::init_pool().await;

    // 2. 앱 라우터 생성 (api 모듈에게 위임 + pool 주입)
    let app = api::app(pool);

    // 3. 서버 실행
    let listener = TcpListener::bind("0.0.0.0:3000").await?;
    println!("🚀 Server started at http://0.0.0.0:3000");
    serve(listener, app).await?;

    Ok(())
}