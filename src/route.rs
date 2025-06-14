use axum::{Router, routing::{ post}};
use sqlx::PgPool;
use crate::handler::{create_user, get_users};

pub fn create_routes(pool: PgPool) -> Router {
    Router::new()
        .route("/users", post(create_user).get(get_users))
        .with_state(pool)
}
