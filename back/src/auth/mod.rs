pub mod handlers;
pub mod models;

use axum::{Router, routing::post};
use sqlx::PgPool;

pub fn router() -> Router<PgPool> {
    Router::new()
        .route("/register", post(handlers::register))
        .route("/login", post(handlers::login))
}
