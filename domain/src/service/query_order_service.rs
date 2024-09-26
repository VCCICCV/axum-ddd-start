use domain::event;
use event::order_event::EventBus;
use common::error::InfraError;
use domain::model::order::Order;
// use domain::repositories::command_order_repository::CommandOrderRepository;
use domain::repositories::query_order_repository::QueryOrderRepository;
/// 查询
pub struct QueryOrderService<Q>
where
    Q: QueryOrderRepository,
{
    repository: Q,
    event_bus: Box<dyn EventBus>, // 任何实现了EventBus trait的对象都可以作为event_bus的参数
}

impl<Q> QueryOrderService<Q>
where
    Q: QueryOrderRepository,
{
    pub fn new(repository: Q, event_bus: Box<dyn EventBus>) -> Self {
        Self {
            repository,
            event_bus,
        }
    }

    pub async fn find_order_by_id(&self, id: i32) -> Result<Option<Order>, InfraError> {
        self.event_bus.publish("OrderFinded");
        self.repository.find_by_id(id).await
    }
}

