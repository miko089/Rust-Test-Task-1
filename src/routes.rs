use axum::extract::{Extension, DefaultBodyLimit};
use sqlx::SqlitePool;
use axum::routing::{get, post};
use crate::handlers;

pub async fn register_routers(app: axum::Router, pool: SqlitePool) -> axum::Router {
    app
        .route("/home", get(handlers::home))
        .route("/create_post", post(handlers::create_post))
        .layer(Extension(pool))
        .layer(DefaultBodyLimit::max(10 * 1024 * 1024)) // set limit to 10mb
}



