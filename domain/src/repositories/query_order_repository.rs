use crate::model::order::Order;
use common::error::InfraError;
#[allow(async_fn_in_trait)]
pub trait QueryOrderRepository {
    fn new() -> Self;
    async fn find_by_id(&self, id: i32) -> Result<Option<Order>, InfraError>;
}
