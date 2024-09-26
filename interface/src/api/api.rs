use axum::{
    http::{HeaderValue, Method},
    response::IntoResponse,
    Router,
};
use std::env;

use crate::routers::user_routes::setup_user_routes;
use crate::{
    common::response::{EmptyData, Res},
    // routers::order_routes::setup_order_routes,
};
use tokio::signal;
use tower_http::cors::CorsLayer;
use tracing::{dispatcher::set_global_default, info};
use tracing_appender::rolling::daily;
use tracing_subscriber::prelude::__tracing_subscriber_SubscriberExt;
use tracing_subscriber::{
    fmt::{self, time::UtcTime},
    EnvFilter, Registry,
};
#[tokio::main]
pub async fn start() -> anyhow::Result<()> {
    // 加载.env 文件，成功返回包含的值，失败返回None
    dotenvy::dotenv().ok();
    // 读取值
    let host = env::var("HOST").expect("HOST is not set in .env file");
    let port = env::var("PORT").expect("PORT is not set in .env file");
    let server_url = format!("{host}:{port}");

    // 读取日志级别
    let log_level = env::var("LOG_LEVEL").unwrap_or("debug".to_string());
    // 设置日志级别过滤器
    let env_filter = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new(log_level))
        .unwrap();
    // 创建每日滚动的日志文件写入器
    let file_appender = daily("logs", "app.log");
    let (non_blocking_appender, guard) = tracing_appender::non_blocking(file_appender);
    // 初始化日志订阅者，同时输出到控制台和文件
    let subscriber = Registry::default()
        .with(env_filter)
        // 控制台输出层
        .with(fmt::layer().with_ansi(true).with_target(false))
        // 文件输出层
        .with(
            fmt::Layer::new()
                .with_writer(non_blocking_appender)
                .with_timer(UtcTime::rfc_3339()),
        );
    set_global_default(subscriber.into()).expect("setting default subscriber failed");

    // 路由以及后备处理
    let app = setup_routes().await.fallback(handler_404);
    // 端口绑定
    let listener = tokio::net::TcpListener::bind(&server_url).await.unwrap();
    // 调用 `tracing` 包的 `info!`，放在启动服务之前，因为会被move
    info!("🚀 listening on {}", &listener.local_addr().unwrap());
    // 启动服务
    axum::serve(listener, app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
    // 在程序结束前释放资源
    drop(guard);
    Ok(())
}
/// 嵌套路由
pub async fn setup_routes() -> Router {
    Router::new()
        .nest("/users", setup_user_routes().await)
        // .nest("/order", setup_order_routes().await)
        //请注意，对于某些请求类型，例如发布content-style：app/json
        //需要添加“.allow_heads（[http：：header：：CONTENT_GROUP]）”
        .layer(
            CorsLayer::new()
                .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
                .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE]),
        )
}
/// 404处理
async fn handler_404() -> impl IntoResponse {
    Res::<EmptyData>::with_not_found()
    // (StatusCode::NOT_FOUND, "nothing to see here")
}
/// 优雅关闭
async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    println!("signal received, starting graceful shutdown");
}
