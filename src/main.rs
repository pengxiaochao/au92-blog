use axum::{middleware as axum_middleware, Router};
use dotenv::dotenv;
use models::Site;
use services::{
    CategoryService, PostService, RssService, SitemapService, TagService, TemplateService,
};
use std::{net::SocketAddr, sync::Arc};
use tracing::Level;
use anyhow::Result;

mod config;
mod db;
mod error;
mod handlers;
mod middleware;
mod models;
mod routes;
mod services;
mod utils;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 加载环境变量
    dotenv().ok();
    // 从环境变量加载站点配置
    let site = Site::from_env();
    // 从环境变量获取日志级别
    let log_level = match std::env::var("LOG_LEVEL")
        .unwrap_or_else(|_| "INFO".to_string())
        .to_uppercase()
        .as_str()
    {
        "TRACE" => Level::TRACE,
        "DEBUG" => Level::DEBUG,
        "INFO" => Level::INFO,
        "WARN" => Level::WARN,
        "ERROR" => Level::ERROR,
        _ => Level::INFO,
    };
    // 初始化日志系统
    tracing_subscriber::fmt().with_max_level(log_level).init();

    let template_service = Arc::new(TemplateService::new()?);
    let post_service = Arc::new(PostService::new(Arc::clone(&template_service)));
    let tag_service = Arc::new(TagService::new(
        Arc::clone(&template_service),
        Arc::clone(&post_service),
    ));
    let category_service = Arc::new(CategoryService::new(
        Arc::clone(&template_service),
        Arc::clone(&post_service),
    ));
    let rss_service = Arc::new(RssService::new(Arc::clone(&post_service), site.clone()));
    let sitemap_service = Arc::new(SitemapService::new(
        Arc::clone(&post_service),
        Arc::clone(&tag_service),
        Arc::clone(&category_service),
        site,
    ));

    // 初始化文章缓存
    post_service.load_all_posts().await?;

    let app = Router::new()
        .merge(routes::create_router(
            category_service,
            tag_service,
            rss_service,
            sitemap_service,
            post_service,
        ))
        .layer(axum_middleware::from_fn(middleware::logging));

    // 从环境变量获取服务器配置
    let host = std::env::var("SERVER_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = std::env::var("SERVER_PORT")
        .unwrap_or_else(|_| "4000".to_string())
        .parse::<u16>()?;
    let addr: SocketAddr = format!("{}:{}", host, port).parse()?;
    println!("Server running on {}", addr);

    // 启动服务器
    let listener = tokio::net::TcpListener::bind(addr).await?;
    tracing::info!("listening on {}", listener.local_addr()?);
    axum::serve(listener, app).await?;

    Ok(())
}
