use axum::{ routing::get, Router };
pub async fn setup_order_routes() -> Router {
    Router::new().route("/", get(show_user))
    // .route("/users/:id", get(get_user))
    // .route("/users", post(create_user))
    // .route("/users", put(update_user))
    // .route("/users/:id", delete(delete_user))
}
async fn show_user() -> String {
    "order".to_string()
}
