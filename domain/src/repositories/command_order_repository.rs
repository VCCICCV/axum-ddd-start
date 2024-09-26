use common::error::InfraError;
use crate::model::order::Order;
#[allow(async_fn_in_trait)]
pub trait CommandOrderRepository {
    async fn create(&self, order: Order) -> Result<Order, InfraError>;
    async fn update(&self,order: Order) -> Result<Order, InfraError>;
    async fn delete(&self, id: i32) -> Result<bool, InfraError>;
    // 其他命令方法
}
