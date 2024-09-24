# Axum启动器

## Stack

- Axum
- Sea-orm
- Tracing
- Tokio
- Tower
- anyhow&thiserror：anyhow捕获错误，thiserror自定义错误

## 迁移

## 启动

## 生成文档

生成markdown文档

```bash
rustdoc README.md
```

生成crate、module文档

```bash
cargo doc --workspace --exclude migration --no-deps
```
