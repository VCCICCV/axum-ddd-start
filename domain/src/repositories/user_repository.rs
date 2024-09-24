use common::error::InfraError;
use crate::model::user::User;
use anyhow::Error;
/// 定义一个UserRepository trait，用于处理用户相关的数据库操作
/// 封装infrastructure层的具体实现
#[allow(async_fn_in_trait)]
pub trait UserRepository {
    async fn find_all(&self) -> Result<Vec<User>, Error>;
    async fn find_by_id(&self, id: i32) -> Result<Option<User>, Error>;
    async fn create(&self, user: User) -> Result<User, InfraError>;
    async fn update(&self,user: User) -> Result<User, Error>;
    async fn delete(&self, id: i32) -> Result<bool, Error>;
}
