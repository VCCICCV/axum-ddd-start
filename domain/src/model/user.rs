use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct User {
    pub id: Option<i32>,
    pub username: String,
    pub email: String,
}