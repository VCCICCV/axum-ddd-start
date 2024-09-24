# axum-ddd-start

旨在提供一个快速搭建Axum+DDD项目的脚手架，我们不希望在技术选型上浪费时间，`axum-ddd-start`主要包含以下内容：

* 领域驱动设计（DDD，Domain Driven Design）
* 整洁架构（Clean Architecture）
* 命令查询职责分离（CQRS，Command Query Responsibility Segregation）

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

运行项目

```bash
ads run
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
