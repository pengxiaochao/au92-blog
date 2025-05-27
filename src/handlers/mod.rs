/// 分类相关处理模块
/// 提供分类列表和分类文章的处理函数
pub mod category;

/// 标签相关处理模块
/// 提供标签列表和标签文章的处理函数
pub mod tag;

/// RSS相关处理模块
/// 提供生成RSS订阅源的处理函数
pub mod rss;

/// Sitemap相关处理模块
/// 提供生成站点地图的处理函数
pub mod sitemap;

/// 归档相关处理模块
/// 提供文章归档和分页显示的处理函数
pub mod archive;

pub mod post;
/// 刷新文章缓存处理模块
pub mod refresh;
pub mod upload;
/// 友链相关处理模块
pub mod friends;

// 导出处理函数，使其可以在其他模块中直接使用
pub use archive::archive_posts;
pub use category::{categories_index, category_posts, category_posts_with_page};
pub use post::{post_detail, render_index};
pub use refresh::refresh_posts;
pub use rss::rss_feed;
pub use sitemap::sitemap_xml;
pub use tag::{tag_posts, tag_posts_with_page, tags_index};
pub use upload::upload_file;
pub use friends::render_friend_links;