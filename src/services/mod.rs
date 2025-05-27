/// 博客核心服务模块
/// 
/// # 模块说明
/// * `post_service` - 文章管理服务，提供文章的CRUD操作
/// * `tag_service` - 标签管理服务，处理文章标签相关功能
/// * `category_service` - 分类管理服务，处理文章分类相关功能
/// * `template_service` - 模板渲染服务，负责HTML页面生成
/// * `rss_service` - RSS订阅服务，生成订阅源
/// * `sitemap_service` - 站点地图服务，生成搜索引擎所需的站点地图

/// 文章服务模块，提供文章的加载、解析和管理功能
pub mod post_service;
/// 标签服务模块，提供标签的统计和文章分类功能
pub mod tag_service;
/// 分类服务模块，提供文章分类管理功能
pub mod category_service;
/// 模板服务模块，提供模板渲染和管理功能
pub mod template_service;
/// RSS服务模块，提供RSS订阅功能
pub mod rss_service;
/// Sitemap服务模块，提供站点地图功能
pub mod sitemap_service;
/// 上传服务模块，提供文件上传功能
pub mod upload_service;
/// 友链服务模块，提供友链的加载和渲染功能
pub mod friend_service;

// 导出服务结构体，方便其他模块使用
pub use post_service::PostService;
pub use tag_service::TagService;
pub use category_service::CategoryService;
pub use template_service::TemplateService;
pub use rss_service::RssService;
pub use sitemap_service::SitemapService;
pub use upload_service::UploadService;
pub use friend_service::FriendLinkService;
