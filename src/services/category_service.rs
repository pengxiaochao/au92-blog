// 导入必要的模块和类型
use super::post_service::PostService;
use super::TemplateService;
use crate::models::CategoryCount;
use crate::models::Page;
use crate::models::Post;
use anyhow::Result;
use std::collections::HashMap;
use std::sync::Arc;
use tera::Context;

/// 分类服务结构体
///
/// # 功能说明
/// - 管理博客文章的分类信息
/// - 提供分类统计和文章过滤功能
/// - 渲染分类相关页面
///
/// # 字段说明
/// * `template_service` - 模板服务实例
/// * `post_service` - 文章服务实例
#[derive(Clone, Debug)]
pub struct CategoryService {
    template_service: Arc<TemplateService>,
    post_service: Arc<PostService>,
}

impl CategoryService {
    /// 创建一个新的 CategoryService 实例
    /// 参数:
    /// - template_service: 模板服务的 Arc 指针
    /// - post_service: 文章服务的 Arc 指针
    pub fn new(template_service: Arc<TemplateService>, post_service: Arc<PostService>) -> Self {
        Self {
            template_service,
            post_service,
        }
    }

    /// 渲染分类列表页面
    ///
    /// # 功能说明
    /// - 获取所有分类及其文章数量
    /// - 渲染分类导航页面
    ///
    /// # 返回
    /// * `Result<String>` - 渲染后的HTML或错误
    pub async fn render_category_page(&self) -> Result<String> {
        let mut context = Context::new();
        let categories = self.get_all_categories().await;
        context.insert("categories", &categories);
        context.insert("count", &categories.len());
        Ok(self
            .template_service
            .render("categories.html.tera", &context)?)
    }

    /// 获取所有分类及其对应的文章数量
    /// 返回按文章数量降序、分类名称升序排序的分类列表
    pub async fn get_all_categories(&self) -> Vec<CategoryCount> {
        // 加载所有文章，如果加载失败则返回空向量
        let posts = self.post_service.load_all_posts().await;
        match posts {
            Ok(posts) => {
                // 使用 HashMap 统计每个分类的文章数量
                let mut category_counts = HashMap::new();
                // 遍历所有文章，统计每个分类的文章数
                for post in posts.into_iter().filter(|p| p.front_matter.draft == false) {
                    if let Some(categories) = post.front_matter.categories {
                        for category in categories {
                            *category_counts.entry(category).or_insert(0) += 1;
                        }
                    }
                }
                // 将 HashMap 转换为 CategoryCount 向量
                let mut categories: Vec<CategoryCount> = category_counts
                    .into_iter()
                    .map(|(name, count)| CategoryCount { name, count })
                    .collect();
                // 按文章数量降序排序，如果数量相同则按分类名称升序排序
                categories.sort_by(|a, b| b.count.cmp(&a.count).then(a.name.cmp(&b.name)));
                categories
            }
            Err(_) => Vec::new(),
        }
    }

    /// 渲染分类下的文章列表
    ///
    /// # 参数
    /// * `category` - 分类名称
    /// * `page` - 当前页码
    ///
    /// # 功能说明
    /// - 获取指定分类下的所有文章
    /// - 实现分页功能
    /// - 渲染文章列表页面
    ///
    /// # 返回
    /// * `Result<String>` - 渲染后的HTML或错误
    pub async fn render_posts_by_category(&self, category: String, page: usize) -> Result<String> {
        let mut context = Context::new();
        // 设置每页显示的记录数
        let per_page: u16 = 20;
        let category_name: String = category.clone();
        let posts = self.get_posts_by_category(category).await;
        let datas: Vec<&Post> = posts
            .iter()
            .skip((page - 1) * (per_page as usize))
            .take(per_page as usize)
            .collect();
        context.insert("posts", &datas);
        context.insert("category_name", &category_name);
        let page = Page::new(posts.len() as u32, page as u16, per_page);
        context.insert("page", &page);
        if page.current > 1 {
            context.insert("site_title", &format!("第{}页 - ", page.current));
        }
        Ok(self
            .template_service
            .render("category_posts.html.tera", &context)?)
    }

    /// 获取指定分类下的所有文章
    /// 参数:
    /// - category: 分类名称
    /// 返回该分类下的所有文章列表
    async fn get_posts_by_category(&self, category: String) -> Vec<Post> {
        // 加载所有文章，如果加载失败则返回空向量
        let posts = self.post_service.load_all_posts().await;
        match posts {
            Ok(datas) => {
                // 筛选出属于指定分类的文章
                datas
                    .into_iter()
                    .filter(|post| post.front_matter.draft == false)
                    .filter(|post| {
                        post.front_matter
                            .categories
                            .as_ref()
                            .map_or(false, |categories| categories.contains(&category))
                    })
                    .collect()
            }
            Err(_) => Vec::new(),
        }
    }
}
