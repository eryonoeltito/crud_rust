
use axum::{
    routing::post,
    Router,
};
use sqlx::PgPool;

use crate::controllers::auth_controller;

pub fn routes() -> Router<PgPool> {
    Router::new()
        .route("/login", post(auth_controller::login))
        .route("/logout", post(auth_controller::logout))
}
