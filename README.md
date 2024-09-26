# axum-ddd-start

旨在提供一个快速搭建Axum+DDD项目的例子，我们不希望在技术选型上浪费时间，`axum-ddd-start`主要包含以下内容：

* 领域驱动设计（DDD，Domain Driven Design）
* 整洁架构（Clean Architecture）
* 事件驱动（EventBus）：基于消息队列的事件驱动架构，一个事件调用，订阅了这个事件的方法也会跟着调用
* 命令查询职责分离（CQRS，Command Query Responsibility Segregation）：将对数据的操作分为命令（Command）和查询（Query）两种不同的职责，并将它们分别处理；命令用于修改数据，查询用于获取数据，二者之间没有直接的耦合关系
  * 可以根据不同操作的特点选择不同的技术和数据存储方式。例如，对于查询操作，可以使用缓存技术或专门的查询数据库来提高查询性能；对于命令操作，可以使用事务性数据库来确保数据的一致性
  * 查询操作可能需要支持复杂的查询条件和分页功能，而命令操作可能需要与其他系统进行集成以实现业务流程的自动化。分别实现可以让这些特定的需求得到更好的满足
* 依赖反转原则（DIP，Dependency Inversion Principle）：高层模块不依赖低层模块，两者都应该依赖于抽象的trait，抽象不应该依赖于细节，细节应该依赖于抽象
* 控制反转（IOC，Inversion of Control）：将模块的创建和使用解耦，使得模块之间的依赖关系更加灵活，易于测试和维护
  * 依赖注入（DI，Dependency Injection）：实现IOC的方式，在一个对象创建时，将其依赖的对象通过构造函数（这里使用`new()`）、属性设置或方法调用等方式注入到该对象中；要使用另一段代码，将其作为参数传入当前代码，而不是直接调用
* 面向切面编程（AOP，Aspect Oriented Programming）：不改变原有代码的情况下添加新功能，将横切关注点（如日志记录、事务管理、权限验证等）与业务逻辑分离，使得业务逻辑更加清晰、易于维护和扩展
* Saga：
* 高并发用线程池还是消息队列？这里选择MQ，主要考虑以下几点：
  * 线程池：自己实现高可用
    * 语言绑定
  * 消息队列MQ：别人实现好了直接用
    * 跨语言

## 请求响应流程

![请求流程](README.assets/%E8%AF%B7%E6%B1%82%E6%B5%81%E7%A8%8B.drawio.png)

* repositories：仓储层，提供一个抽象来访问数据存储
  * 封装数据访问逻辑
  * 提供统一的访问接口
  * 与数据存储解耦
* persistence：持久层，将数据持久化到存储介质中，如数据库、文件系统等
  * 关注数据如何保存和加载
  * 与业务逻辑解耦

## 技术栈

* Axum：Web框架
* Sea-orm：ORM
* Tracing：日志管理
* Tokio：异步运行时
* Tower
* anyhow&thiserror：anyhow捕获错误，thiserror自定义错误

## 快速开始

安装

```bash
cargo install ads-cli
```

创建项目

```bash
ads new
```

使用

```bash
USAGE:
    ads [OPTIONS] <SUBCOMMAND>
OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information

SUBCOMMANDS:
    help           Print this message or the help of the given subcommand(s)
    new            Create a new project
```

## 启动示例项目

clone本项目，根据自己的项目进行修改

## 部署数据库

```bash
docker-compose up -d
```

## 迁移示例项目

安装

```bash
cargo install sea-orm-cli
```

迁移

```bash
sea-orm-cli migrate up
```

回滚

```bash
sea-orm-cli migrate down
```

删除所有表重新迁移

```bash
sea-orm-cli migrate fresh
```

## 示例项目启动

```bash
cargo run
```

## 生成文档

生成markdown文档

```bash
rustdoc README.md
```

生成crate、module文档

```bash
cargo doc --workspace --exclude migration --no-deps
```

## EventBus（待完成）

通过发布订阅模式实现事件驱动架构，主要包含以下内容：

* 1、定义事件 Event
* 2、订阅者 Subscribe
  * 注册 Register：接收一个事件对象
  * 取消订阅
* 3、发布者 Publish
假设我们有订单模块和库存模块两个微服务，当一个订单被创建时，库存模块订阅事件并减少库存，两者不需要直接知道对方的存在，子需要官族自己的业务逻辑和发布订阅事件
![EventBus-Publish-Subscribe.png](README.assets/EventBus-Publish-Subscribe.png)
