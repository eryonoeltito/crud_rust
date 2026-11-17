use axum::{Router, routing::post};
use sqlx::PgPool;

use crate::controllers::auth_controller;

pub fn routes() -> Router<PgPool> {
    Router::new()
        .route("/api/auth/register", post(auth_controller::register))
        .route("/api/auth/login", post(auth_controller::login))
        .route("/api/auth/logout", post(auth_controller::logout))
}
