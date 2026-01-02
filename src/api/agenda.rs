use axum::{Router, routing::get};

pub fn router() -> Router<sqlx::PgPool> {
    Router::new()
        .route("/", get(list_agendas)) // GET /agendas/
}

async fn list_agendas() -> &'static str {
    "Agenda List"
}