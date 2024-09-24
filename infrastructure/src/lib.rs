//! 基础设施层
pub mod database{
    pub mod connection;
}
pub mod entities{
    pub mod usernake;
    pub mod prelude;
    pub mod user;
}
pub mod persistence{
    pub mod user_repository_impl;
}
