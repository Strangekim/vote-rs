use axum::Router;
use sqlx::PgPool;

mod dtos;
mod handlers;
mod repository;
mod service;
#[cfg(test)]
mod test;

pub fn router() -> Router<PgPool> {
    use axum::routing::post;
    use handlers::create_agenda;

    Router::new()
        .route("/", post(create_agenda))
}
