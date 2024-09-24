use axum::response::IntoResponse;

pub use application::service::user_service::UserService;
pub use domain::model::user::User;

use crate::common::response::Res;

pub async fn list_users() -> impl IntoResponse {
    let users = UserService::new().find_all_users().await.unwrap();
    // json!(users).to_string()
    Res::with_data(users)
}

pub async fn get_user(id: axum::extract::Path<i32>) -> impl IntoResponse {
    let user = UserService::new().find_user_by_id(*id).await.unwrap();
    // json!(user).to_string()
    Res::with_data(user)
}

pub async fn create_user(user: axum::extract::Json<User>) -> impl IntoResponse {
    let result = UserService::new().create_user(user.0).await;
    // match result {
    //     Ok(new_user) => Res::with_data(new_user),
    //     Err(err) => Res::with_err(&err.to_string()),
    // }
    match result {
        Ok(new_user) => Res::with_data(new_user),
        Err(err) => {
            // Convert InfraError to a string for logging or user feedback
            let error_message = err.to_string();
            Res::with_err(&error_message)
        }
    }
}

pub async fn update_user(user: axum::extract::Json<User>) -> impl IntoResponse {
    let result = UserService::new().update_user(user.0).await;
    match result {
        Ok(new_user) => Res::with_data(new_user),
        Err(err) => Res::with_err(&err.to_string()),
    }
}

pub async fn delete_user(id: axum::extract::Path<i32>) -> impl IntoResponse {
    let result = UserService::new().delete_user(*id).await;
    match result {
        Ok(deleted) => {
            if deleted {
                Res::with_data("User deleted successfully")
            } else {
                Res::with_err("User not found or deletion failed")
            }
        }
        Err(err) => Res::with_err(&err.to_string()),
    }
}
