/// 路由模块 - 负责处理所有HTTP路由配置和请求分发
use crate::services::{CategoryService, PostService, RssService, SitemapService};
use crate::{handlers, services::TagService};
use axum::routing::get_service;
use axum::{routing::get, Router};
use std::sync::Arc;
use tower_http::services::ServeDir;
/// 应用程序共享状态
/// 包含所有服务实例，通过 Arc 实现线程安全的共享
/// 在请求处理过程中可以访问这些服务
#[derive(Clone)]
pub struct AppState {
    /// 分类服务实例：处理文章分类相关功能
    pub category_service: Arc<CategoryService>,
    /// 标签服务实例：处理文章标签相关功能
    pub tag_service: Arc<TagService>,
    /// RSS服务实例：生成站点RSS订阅源
    pub rss_service: Arc<RssService>,
    /// Sitemap服务实例：生成站点地图
    pub sitemap_service: Arc<SitemapService>,
    /// 文章服务实例：处理文章的加载和管理
    pub post_service: Arc<PostService>,
}

/// 创建并配置应用路由系统
///
/// # 功能说明
/// - 初始化应用状态，包含所有核心服务实例
/// - 配置所有HTTP路由规则和对应的处理函数
/// - 支持文章、分类、标签、RSS和站点地图等功能的访问
///
/// # 路由说明
/// - `/post/page/:page/` - 分页显示文章列表
/// - `/post/` - 显示文章首页
/// - `/categories/` - 显示分类列表
/// - `/categories/:category/` - 显示特定分类下的文章
/// - `/tags/` - 显示标签云
/// - `/tags/:tag/` - 显示特定标签下的文章
/// - `/index.xml` - RSS订阅源
/// - `/sitemap.xml` - 网站地图
/// - `/refresh/posts/` - 刷新文章缓存
pub fn create_router(
    category_service: Arc<CategoryService>,
    tag_service: Arc<TagService>,
    rss_service: Arc<RssService>,
    sitemap_service: Arc<SitemapService>,
    post_service: Arc<PostService>,
) -> Router {
    // 创建应用状态实例
    let state = AppState {
        category_service,
        tag_service,
        rss_service,
        sitemap_service,
        post_service,
    };

    // 构建路由表
    // 使用 axum 的 Router 来定义所有路由规则
    Router::new()
        // 文章相关路由 - 注意顺序调整
        .route("/post/page/{page}/", get(handlers::archive_posts))
        .route("/post/", get(handlers::archive_posts))
        .route("/post/{url}/", get(handlers::post_detail))
        .route("/post/{url}/index.html", get(handlers::post_detail))
        // 分类相关路由
        .route("/categories/", get(handlers::categories_index))
        .route("/categories/{category}/page/{page}/", get(handlers::category_posts_with_page))
        .route("/categories/{category}/", get(handlers::category_posts))
        // 标签相关路由
        .route("/tags/", get(handlers::tags_index))
        .route("/tags/{tag}/page/{page}/", get(handlers::tag_posts_with_page))
        .route("/tags/{tag}/", get(handlers::tag_posts))
        // 站点功能路由
        .route("/index.xml", get(handlers::rss_feed))
        .route("/sitemap.xml", get(handlers::sitemap_xml))
        // 首页路由
        .route("/index.html", get(handlers::render_index))
        .route("/", get(handlers::render_index))
        .route("/page/{page}/", get(handlers::render_index))
        // 管理功能路由
        .route("/refresh/posts/", get(handlers::refresh_posts))
        // Static files
        .nest_service("/static", get_service(ServeDir::new("static")))
        // 注入应用状态
        .with_state(state)
}
