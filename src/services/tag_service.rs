use super::post_service::PostService;
use super::TemplateService;
use crate::models::Page;
use crate::models::Post;
use crate::models::TagCount;
use anyhow::Result;
use std::collections::HashMap;
use std::sync::Arc;
use tera::Context;

/// 标签服务结构体
///
/// # 功能说明
/// - 提供标签统计、查询和渲染功能
/// - 管理文章标签的聚合和展示
///
/// # 字段说明
/// * `template_service` - 模板服务实例，用于渲染标签相关页面
/// * `post_service` - 文章服务实例，用于获取文章数据
#[derive(Clone, Debug)]
pub struct TagService {
    /// 模板服务实例
    template_service: Arc<TemplateService>,
    /// 文章服务实例
    post_service: Arc<PostService>,
}

impl TagService {
    /// 创建标签服务实例
    ///
    /// # 参数
    /// * `template_service` - 模板服务Arc指针
    /// * `post_service` - 文章服务Arc指针
    ///
    /// # 返回
    /// * `Self` - 标签服务实例
    pub fn new(template_service: Arc<TemplateService>, post_service: Arc<PostService>) -> Self {
        Self {
            template_service,
            post_service,
        }
    }

    /// 渲染标签列表页面
    ///
    /// # 功能说明
    /// - 获取所有标签及其使用频次
    /// - 渲染标签云页面
    ///
    /// # 返回
    /// * `Result<String>` - 渲染后的HTML或错误
    pub async fn render_tag_page(&self) -> Result<String> {
        let mut context = Context::new();
        let tags = self.get_all_tags().await;
        context.insert("tags", &tags);
        context.insert("count", &tags.len());
        self.template_service.render("tags.html.tera", &context)
    }

    /// 获取所有标签及其使用次数
    ///
    /// # 功能说明
    /// - 统计所有非草稿文章中的标签使用频次
    /// - 按使用频次降序、标签名升序排序
    ///
    /// # 返回
    /// * `Vec<TagCount>` - 标签统计列表
    pub async fn get_all_tags(&self) -> Vec<TagCount> {
        let posts = self.post_service.load_all_posts().await;
        match posts {
            Ok(posts) => {
                let mut tag_counts = HashMap::new();
                for post in posts.into_iter().filter(|p| !p.front_matter.draft) {
                    if let Some(tags) = post.front_matter.tags {
                        for tag in tags {
                            *tag_counts.entry(tag).or_insert(0) += 1;
                        }
                    }
                }
                let mut tags: Vec<TagCount> = tag_counts
                    .into_iter()
                    .map(|(name, count)| TagCount { name, count })
                    .collect();
                tags.sort_by(|a, b| b.count.cmp(&a.count).then(a.name.cmp(&b.name)));
                tags
            }
            Err(_) => Vec::new(),
        }
    }

    /// 渲染指定标签的文章列表
    ///
    /// # 参数
    /// * `tag` - 标签名称
    /// * `page` - 页码，从1开始
    ///
    /// # 功能说明
    /// - 获取指定标签下的所有文章
    /// - 实现分页功能
    /// - 渲染文章列表页面
    ///
    /// # 返回
    /// * `Result<String>` - 渲染后的HTML或错误
    pub async fn render_posts_by_tag(&self, tag: String, page: usize) -> Result<String> {
        let mut context = Context::new();
        // 设置每页显示的记录数
        let per_page: u16 = 20;
        let tag_name: String = tag.clone();
        let posts = self.get_posts_by_tag(tag).await;
        let datas: Vec<&Post> = posts
            .iter()
            .skip((page - 1) * (per_page as usize))
            .take(per_page as usize)
            .collect();
        context.insert("posts", &datas);
        context.insert("tag_name", &tag_name);
        let page = Page::new(posts.len() as u32, page as u16, per_page);
        context.insert("page", &page);
        if page.current > 1 {
            context.insert("site_title", &format!("第{}页 - ", page.current));
        }
        self.template_service
            .render("tag_posts.html.tera", &context)
    }

    /// 获取指定标签下的所有文章
    ///
    /// # 参数
    /// * `tag` - 标签名称
    ///
    /// # 功能说明
    /// - 过滤出包含指定标签的非草稿文章
    ///
    /// # 返回
    /// * `Vec<Post>` - 文章列表
    pub async fn get_posts_by_tag(&self, tag: String) -> Vec<Post> {
        let posts = self.post_service.load_all_posts().await;
        match posts {
            Ok(posts) => posts
                .into_iter()
                .filter(|p| !p.front_matter.draft)
                .filter(|post| {
                    post.front_matter
                        .tags
                        .as_ref()
                        .is_some_and(|tags| tags.contains(&tag))
                })
                .collect(),
            Err(_) => Vec::new(),
        }
    }
}
