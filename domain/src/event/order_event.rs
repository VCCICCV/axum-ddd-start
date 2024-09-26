
pub trait EventBus {
    fn publish(&self, event: &str);
    fn subscribe(&mut self, event_type: String, subscriber: Box<dyn Fn(String) + Send + Sync>);
}
