use crate::models::{Site, Sitemap, SitemapUrl};
use crate::services::{PostService, TagService, CategoryService};
use chrono::{DateTime, FixedOffset, Local};
use std::sync::Arc;
use anyhow::Result;

/// 站点地图服务
/// 
/// # 功能说明
/// - 生成符合搜索引擎规范的站点地图
/// - 包含所有页面的URL集合
/// - 提供页面的最后更新时间和优先级
/// 
/// # 字段说明
/// * `post_service` - 文章服务实例
/// * `tag_service` - 标签服务实例
/// * `category_service` - 分类服务实例
/// * `site` - 网站配置信息
pub struct SitemapService {
    post_service: Arc<PostService>,
    tag_service: Arc<TagService>,
    category_service: Arc<CategoryService>,
    site: Site,
}

impl SitemapService {
    /// 创建站点地图服务实例
    /// 
    /// # 参数
    /// * `post_service` - 文章服务Arc指针
    /// * `tag_service` - 标签服务Arc指针
    /// * `category_service` - 分类服务Arc指针
    /// * `site` - 站点配置信息
    /// 
    /// # 返回
    /// * `Self` - 站点地图服务实例
    pub fn new(
        post_service: Arc<PostService>,
        tag_service: Arc<TagService>,
        category_service: Arc<CategoryService>,
        site: Site,
    ) -> Self {
        Self {
            post_service,
            tag_service,
            category_service,
            site,
        }
    }

    /// 格式化日期时间为ISO 8601格式
    /// 
    /// # 参数
    /// * `dt` - 待格式化的日期时间
    /// 
    /// # 返回
    /// * `String` - ISO 8601格式的日期时间字符串
    fn format_datetime(dt: DateTime<FixedOffset>) -> String {
        dt.format("%Y-%m-%dT%H:%M:%S%z").to_string()
    }

    /// 生成站点地图XML
    /// 
    /// # 功能说明
    /// - 收集所有页面的URL
    /// - 添加最后更新时间和优先级
    /// - 生成符合规范的XML文档
    /// 
    /// # 返回
    /// * `Result<String>` - XML格式的站点地图或错误
    pub async fn generate_sitemap_xml(&self) -> Result<String> {
        let mut urls = Vec::new();
        let current_time = Self::format_datetime(Local::now().with_timezone(&Local).into());

        // Add home page
        urls.push(SitemapUrl {
            loc: self.site.url.clone(),
            lastmod: current_time.clone(),
            priority: self.site.priority.clone(),
        });

        // Add posts
        let posts = self.post_service.load_all_posts().await?;
        for post in posts {
            if post.front_matter.draft {
                continue;
            }
            urls.push(SitemapUrl {
                loc: format!("{}/post/{}/", self.site.url, post.url),
                lastmod: Self::format_datetime(post.front_matter.date),
                priority: self.site.priority.clone(),
            });
        }

        // Add tag pages
        let tags = self.tag_service.get_all_tags().await;
        for tag in tags {
            urls.push(SitemapUrl {
                loc: format!("{}/tags/{}/", self.site.url, tag.name),
                lastmod: current_time.clone(),
                priority: self.site.priority.clone(),
            });
        }

        // Add category pages
        let categories = self.category_service.get_all_categories().await;
        for category in categories {
            urls.push(SitemapUrl {
                loc: format!("{}/categories/{}/", self.site.url, category.name),
                lastmod: current_time.clone(),
                priority: self.site.priority.clone(),
            });
        }

        let sitemap = Sitemap { urls };
        self.render_sitemap_xml(&sitemap)
    }

    /// 将站点地图数据渲染为XML
    /// 
    /// # 参数
    /// * `sitemap` - 站点地图数据结构
    /// 
    /// # 返回
    /// * `Result<String>` - 渲染后的XML字符串
    fn render_sitemap_xml(&self, sitemap: &Sitemap) -> Result<String> {
        let mut xml = String::from(r#"<?xml version="1.0" encoding="UTF-8"?>"#);
        xml.push_str(r#"<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">"#);
        for url in &sitemap.urls {
            xml.push_str(&format!(
                r#"<url><loc>{}</loc><lastmod>{}</lastmod><priority>{}</priority></url>"#,
                url.loc, url.lastmod, url.priority
            ));
        }

        xml.push_str("</urlset>");
        Ok(xml)
    }
}
