use axum::{routing::{ delete, get, post, put }, Router};
use crate::adapter::handler::{ create_user, delete_user, get_user, list_users, update_user };
pub async fn setup_user_routes() -> Router {
    Router::new()
        .route("/", get(list_users))
        .route("/:id", get(get_user))
        .route("/", post(create_user))
        .route("/", put(update_user))
        .route("/:id", delete(delete_user))
}