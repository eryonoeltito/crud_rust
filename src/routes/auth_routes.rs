use axum::{Router, routing::post};
use sqlx::PgPool;

use crate::controllers::auth_controller;

pub fn routes() -> Router<PgPool> {
    Router::new()
        .route("/register", post(auth_controller::register))
        .route("/login", post(auth_controller::login))
        .route("/logout", post(auth_controller::logout))
}
