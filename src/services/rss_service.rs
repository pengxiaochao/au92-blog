// RSS服务模块：负责生成网站的RSS订阅源
use crate::models::{RssFeed, RssItem, Site};
use crate::services::PostService;
use crate::utils::html::escape_html;
use anyhow::Result;
use chrono::{DateTime, FixedOffset};
use std::sync::Arc;

/// RSS服务模块
///
/// # 功能说明
/// - 生成博客的RSS订阅源
/// - 将最新文章转换为RSS格式
/// - 支持文章内容的摘要生成
///
/// # 字段说明
/// * `post_service` - 文章服务实例，用于获取最新文章
/// * `site` - 网站配置信息，包含RSS相关设置
pub struct RssService {
    post_service: Arc<PostService>,
    site: Site,
}

impl RssService {
    /// 创建RSS服务实例
    ///
    /// # 参数
    /// * `post_service` - 文章服务Arc指针
    /// * `site` - 网站配置信息
    ///
    /// # 返回
    /// * `Self` - RSS服务实例
    pub fn new(post_service: Arc<PostService>, site: Site) -> Self {
        Self { post_service, site }
    }

    /// 生成RSS订阅源
    ///
    /// # 功能说明
    /// - 获取最新的指定数量文章
    /// - 转换为RSS条目格式
    /// - 添加订阅源元数据
    ///
    /// # 返回
    /// * `Result<RssFeed>` - RSS订阅源数据或错误
    pub async fn generate_feed(&self) -> Result<RssFeed> {
        // 加载所有文章
        let posts = self.post_service.load_all_posts().await?;

        // 获取所有文章，转换为RSS项目格式
        let items: Vec<RssItem> = posts
            .iter()
            .filter(|p| !p.front_matter.draft)
            // .take(self.site.rss_count)
            .map(|post| RssItem {
                title: post.front_matter.title.clone(),
                link: format!("{}/post/{}/", self.site.url, post.url),
                pub_date: self.format_datetime(post.front_matter.date),
                description: escape_html(post.generate_description(200).as_str()),
            })
            .collect();

        // 获取当前时间作为RSS最后更新时间
        // let last_build_date = self.format_datetime(Local::now().with_timezone(&Local).into());
        // 获取最后一篇文章的时间作为最后更新时间
        let last_build_date = self.format_datetime(posts.first().unwrap().front_matter.date);

        // 构建并返回RSS Feed
        Ok(RssFeed {
            items,
            last_build_date,
            site_url: self.site.url.clone(),
            site_title: self.site.title.clone(),
        })
    }

    /// 格式化日期时间为RSS规范格式
    ///
    /// # 参数
    /// * `dt` - 待格式化的日期时间
    ///
    /// # 返回
    /// * `String` - 格式化后的日期时间字符串
    fn format_datetime(&self, dt: DateTime<FixedOffset>) -> String {
        dt.format("%Y-%m-%dT%H:%M:%S%z").to_string()
    }

    /// 生成RSS XML内容
    ///
    /// # 功能说明
    /// - 生成符合RSS 2.0规范的XML文档
    /// - 包含频道信息和文章条目
    ///
    /// # 返回
    /// * `Result<String>` - RSS XML字符串或错误
    pub async fn generate_feed_xml(&self) -> Result<String> {
        let feed = self.generate_feed().await?;

        let xml = format!(
            r#"<?xml version="1.0" encoding="utf-8" standalone="yes"?>
<rss version="2.0" xmlns:atom="http://www.w3.org/2005/Atom">
  <channel>
    <title>{}</title>
    <link>{}</link>
    <description>Recent content on P.X.C</description>
    <generator>P.X.C Blog Engine</generator>
    <language>zh-cn</language>
    <lastBuildDate>{}</lastBuildDate>
    <atom:link href="{}/index.xml" rel="self" type="application/rss+xml" />"#,
            feed.site_title, feed.site_url, feed.last_build_date, feed.site_url
        );

        let items: String = feed
            .items
            .iter()
            .map(|item| {
                format!(
                    r#"    <item>
      <title>{}</title>
      <link>{}</link>
      <pubDate>{}</pubDate>
      <guid>{}</guid>
      <description>{}</description>
    </item>"#,
                    item.title, item.link, item.pub_date, item.link, item.description
                )
            })
            .collect();

        Ok(format!("{}\n{}\n  </channel>\n</rss>", xml, items))
    }
}
