//! 领域层
pub mod model {
    pub mod order;
    pub mod user;
}
/// 高层Domain不应该依赖于低层Infrastructure，而是应该依赖于抽象trait
pub mod repositories {
    /// 1、不实现CQRS
    // pub mod user_repository;
    /// 2、CQRS，将存储库的操作抽象为命令和查询
    pub mod command_user_repository;
    pub mod query_user_repository;
    // pub mod command_order_repository;
    // pub mod query_order_repository;
}
/// 领域服务（领域能力）：这个领域提供的能力，比如提供了删除的能力，如果要判断有没有权限删除，那就在application编排先鉴权、再删除的用例，
/// 领域服务是领域层的核心，它应该是无状态的，并且不应该依赖于任何其他领域层的组件，应该通过repository来获取数据
pub mod service {
    pub mod command_user_service;
    pub mod query_user_service;
}
/// 值对象：没有唯一标识的对象，由其属性的值定义，通常是不可变的
/// 比如，地址可以作为一个值对象。地址由国家、省份、城市、街道、邮编等属性组成，这些属性的值共同定义了一个地址
/// 如果两个地址的所有属性的值都相同，那么这两个地址就是相等的
pub mod value_object {}

/// 聚合：聚合是由多个实体和值对象组成的，聚合的根实体负责协调聚合内的实体之间的关系，聚合的根实体是聚合内的唯一标识
/// 聚合内部的实体和值对象只能通过聚合根进行访问，外部对象只能通过聚合根来操作聚合内部的元素
/// 订单可以作为一个聚合。订单由订单头（包含订单编号、客户信息、订单状态等）和订单行项（包含商品信息、数量、价格等）组成
/// 订单头是聚合根，它负责管理订单行项的生命周期
/// 外部对象只能通过订单头来操作订单行项，例如添加、删除或修改订单行项
pub mod aggregate {}

/// 定义事件总线的trait，表示发生的事件，在infrastructure层中实现
/// 通过发布订阅（观察者模式）模式，解决业务之间的耦合，订阅者之间互不认识也不干扰
/// 订单创建成功后，发布一个订单注册成功事件，订阅者根据需要订阅订单组测成功事件，编写相对应的处理程序
///
/// 事件总线类型
/// * 跨进程事件总线（集成事件总线）：发布者与订阅者不在同一个进程中，订阅者是一个新的请求
/// * 跨服务事件总线（消息队列）：发布者与订阅者不在同一个服务中，订阅者是一个新的请求
/// * 进程内事件总线（领域事件总线）：发布者与订阅者在同一个进程中，订阅者是出错会引起当前请求出错
pub mod event {
    // 定义事件处理的trait，用于处理特定类型的事件
    pub mod user_event;
    // 事件总线侧重于业务领域事件的传播和处理
    pub mod event_bus {
        pub mod user_event_bus;
    }
}
