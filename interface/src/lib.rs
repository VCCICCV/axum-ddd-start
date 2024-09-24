//! 应用程序的入口点，用于启动应用程序
pub mod api{
    pub mod api;
}
/// 为什么要多加一层apapter？
/// CQRS职责分离，这里的目的是为了将业务逻辑和HTTP请求处理解耦，使应用程序的核心逻辑更加清晰，易于测试和维护
/// 将从业务逻辑层获取的数据转换为适合接口（例如 HTTP 响应）的格式
/// 只关注如何与外部进行交互，而不涉及具体的业务逻辑的实现细节
pub mod adapter{
    pub mod handler;
}
/// 公共响应
pub mod common{
    pub mod response;
}
/// 路由
pub mod routers{
    pub mod user_routes;
    pub mod order_route;
}
