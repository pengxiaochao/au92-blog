use super::TemplateService;
use crate::error::AppError;
use crate::models::page::Page;
use crate::models::{Archive, ArchivePost, FrontMatter, Post};
use anyhow::Result;
use chrono::Datelike;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::{path::Path, sync::Arc};
use tera::Context;
use tokio::fs;
use tokio::sync::RwLock;
// 添加 AppError 导入
/// 全局文章缓存
/// 使用 Lazy 和 RwLock 实现线程安全的延迟初始化缓存
static POSTS_CACHE: Lazy<RwLock<Option<Vec<Post>>>> = Lazy::new(|| RwLock::new(None));
/// 定义每分钟阅读汉字的速度，用于计算文章阅读时间
static READ_SPEED: u16 = 200; // 阅读速度（汉字/分钟）

/// 文章服务结构体
/// 负责博客文章的加载、缓存管理、解析和查询等核心功能
/// PostService结构体: 负责所有与博客文章相关的核心业务逻辑
///
/// # 主要职责
/// - 文章的加载和解析：从文件系统读取和解析 Markdown 文件
/// - 文章缓存的管理：实现高效的文章访问机制
/// - 文章的查询和渲染：支持按URL查找和渲染文章
/// - 首页和文章详情页的数据准备：处理分页和文章详情展示
#[derive(Clone, Debug)]
pub struct PostService {
    /// 模板服务实例，用于处理页面渲染
    template_service: Arc<TemplateService>,
}

/// 文章摘要结构体，用于首页文章列表展示
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PostSummary {
    pub front_matter: FrontMatter, // 文章元数据，包含标题、日期等信息
    pub content: String,           // 文章完整内容
    pub url: String,              // 文章访问地址
    pub summary: String,          // 文章摘要内容
    pub count: usize,             // 文章字数统计
    pub read_time: u16,           // 预估阅读时间（分钟）
}

/// 单篇文章详情结构体，用于文章详情页展示
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SinglePost {
    pub front_matter: FrontMatter,         // 文章元数据
    pub content: String,                   // 文章HTML内容
    pub summary: String,                   // 文章摘要
    pub url: String,                       // 文章URL
    pub count: usize,                      // 文章字数
    pub read_time: u16,                    // 预估阅读时间
    pub toc: Vec<(usize, String, String)>, // 文章目录结构：(层级, 标题, ID)
    pub prev: Option<Post>,                // 上一篇文章
    pub next: Option<Post>,                // 下一篇文章
}

impl PostService {
    /// 创建新的文章服务实例
    ///
    /// # 参数
    /// * `template_service` - 模板服务实例
    pub fn new(template_service: Arc<TemplateService>) -> Self {
        Self { template_service }
    }

    /// 渲染首页
    /// * `page` - 页码
    ///
    /// # 功能说明
    /// - 加载并过滤非草稿状态的文章
    /// - 处理分页逻辑
    /// - 生成文章摘要和阅读时间
    /// - 准备模板渲染所需的上下文数据
    ///
    /// # 参数
    /// * `page` - 当前页码，从1开始
    ///
    /// # 返回值
    /// * `Result<String>` - 渲染后的HTML字符串
    pub async fn render_index(&self, page: usize) -> Result<String> {
        let mut context = Context::new();
        // 设置每页显示的记录数
        let per_page = 10;
        // 加载所有文章
        let all_posts = self
            .load_all_posts()
            .await?
            .into_iter()
            .filter(|p| p.front_matter.draft == false);
        // 使用count()获取过滤后的数量
        let len = all_posts.clone().count();
        let posts: Vec<PostSummary> = all_posts
            .skip((page-1) * per_page)
            .take(per_page)
            .map(|post| {
                let summary = post.generate_description(200);
                PostSummary {
                    front_matter: post.front_matter.clone(),
                    content: post.content.clone(),
                    url: post.url.clone(),
                    summary,
                    count: post.count_chinese_chars(),
                    read_time: post.read_time(READ_SPEED),
                }
            })
            .collect();
        context.insert("posts", &posts);

        let total_pages = (len + per_page - 1) / per_page;
        let page = Page::from_count(total_pages as u16, page as u16);
        context.insert("page", &page);

        if page.current > 1 {
            context.insert("site_title", &format!("第{}页 - ", page.current));
        }

        // 渲染首页模板
        self.template_service.render("index.html.tera", &context)
    }

    /// 渲染博客文章详情页面
    ///
    /// # 功能说明
    /// - 根据URL查找对应的文章
    /// - 如果找到文章，使用模板渲染文章详情页
    /// - 如果未找到文章，返回None
    ///
    /// # 参数
    /// * `url` - 文章的唯一标识符
    ///
    /// # 返回值
    /// * `Result<String, AppError>` - 成功时返回渲染后的HTML字符串，文章不存在时返回错误
    pub async fn render_post(&self, url: &str) -> Result<String, AppError> {
        // 尝试获取指定URL的文章
        let post = self.get_post(url).await?;
        match post {
            Some(post) => {
                let mut context = Context::new();
                context.insert("post", &post);
                if let Some(tags) = &post.front_matter.tags {
                    context.insert("keywords", &tags.join(","));
                }
                context.insert("description", &post.summary);
                Ok(self.template_service.render("single.html.tera", &context)?)
            }
            None => Err(AppError::NotFound(url.to_string())),
        }
    }

    /// 获取所有文章（优先从缓存获取）
    ///
    /// # 功能说明
    /// - 实现了三级缓存检查机制
    /// - 使用读写锁确保并发安全
    /// - 支持缓存失效时重新加载
    ///
    /// # 实现细节
    /// 1. 快速路径：尝试读取缓存
    /// 2. 缓存未命中：获取写锁并加载
    /// 3. 写锁被占用：等待其他线程完成加载
    ///
    /// # 返回值
    /// * `Result<Vec<Post>>` - 所有文章的集合
    pub async fn load_all_posts(&self) -> Result<Vec<Post>> {
        // 1. 快速路径：尝试读取缓存
        if let Ok(guard) = POSTS_CACHE.try_read() {
            if let Some(posts) = guard.as_ref() {
                return Ok(posts.clone());
            }
        }

        // 2. 缓存未命中：尝试获取写锁并加载
        match POSTS_CACHE.try_write() {
            Ok(mut guard) => {
                // 双重检查
                if let Some(posts) = guard.as_ref() {
                    return Ok(posts.clone());
                }

                // 加载文章
                let posts = self.load_posts_from_fs().await?;
                *guard = Some(posts.clone());
                Ok(posts)
            }
            // 3. 无法获取写锁：等待其他线程完成加载
            Err(_) => {
                let guard = POSTS_CACHE.read().await;
                match guard.as_ref() {
                    Some(posts) => Ok(posts.clone()),
                    None => {
                        // 以防万一：如果仍然为空，加载文件系统
                        let posts = self.load_posts_from_fs().await?;
                        drop(guard);
                        let mut write_guard = POSTS_CACHE.write().await;
                        *write_guard = Some(posts.clone());
                        Ok(posts)
                    }
                }
            }
        }
    }


    /// 刷新文章缓存
    /// # 功能说明
    /// - 从文件系统加载并解析文章信息，然后更新缓存
    /// # 返回
    /// - 成功返回 Ok(())
    /// - 失败返回错误
    pub async fn refresh(&self) -> Result<()> {
        let posts = self.load_posts_from_fs().await?;
        let mut cache = POSTS_CACHE.write().await;
        *cache = Some(posts);
        Ok(())
    }

    /// 渲染归档列表
    /// # 功能说明
    /// - 获取分页参数，根据当前页和每页文章数进行分页
    /// - 从服务中获取按年份分组的文章归档并进行渲染
    /// # 参数
    /// * `page` - 当前页码
    /// * `per_page` - 每页文章数量
    /// # 返回
    /// * `Result<String>` - 返回渲染后的HTML字符串或错误
    pub async fn render_archives(&self, page: usize, per_page: usize) -> Result<String> {
        let mut context = Context::new();
        let (archives, len) = self.get_paginated_archives(page, per_page).await?;
        context.insert("archives", &archives);

        let page = Page::new(len as u32, page as u16, per_page as u16);
        context.insert("page", &page);

        if page.current > 1 {
            context.insert("site_title", &format!("第{}页 - ", page.current));
        }
        Ok(self
            .template_service
            .render("archives.html.tera", &context)?)
    }


    /// 获取单篇文章
    ///
    /// # 功能说明
    /// - 从已加载的文章列表中查找指定URL的文章
    /// - 过滤掉草稿状态的文章
    ///
    /// # 参数
    /// * `url` - 文章的唯一标识符
    ///
    /// # 返回值
    /// * `Result<Option<Post>>` - 找到文章时返回Some(Post)，否则返回None
    async fn get_post(&self, url: &str) -> Result<Option<SinglePost>> {
        let posts = self.load_all_posts().await?;

        // 过滤出非草稿文章
        let published_posts: Vec<Post> = posts
            .into_iter()
            .filter(|p| p.front_matter.draft == false)
            .collect();

        // 查找当前文章的索引
        if let Some(current_index) = published_posts.iter().position(|p| p.url == url) {
            let current_post = &published_posts[current_index];

            // 获取下一篇文章（如果存在）
            let next = if current_index + 1 < published_posts.len() {
                Some(published_posts[current_index + 1].clone())
            } else {
                None
            };

            // 获取上一篇文章（如果存在）
            let prev = if current_index > 0 {
                Some(published_posts[current_index - 1].clone())
            } else {
                None
            };

            // 构造SinglePost对象
            Ok(Some(SinglePost {
                front_matter: current_post.front_matter.clone(),
                url: current_post.url.clone(),
                content: current_post.generate_html(),
                summary: current_post.generate_description(100),
                count: current_post.count_chinese_chars(),
                read_time: current_post.read_time(READ_SPEED),
                toc: current_post.generate_toc(),
                prev,
                next,
            }))
        } else {
            Ok(None)
        }
    }


    /// 加载并解析单个Markdown文章文件
    ///
    /// # 参数
    /// * `path` - 文章文件路径
    ///
    /// # 返回
    /// * `Result<Post>` - 解析后的文章对象或错误
    ///
    /// # 错误
    /// * 文件读取失败时返回错误
    /// * Front Matter解析失败时返回错误
    async fn load_post<P: AsRef<Path>>(&self, path: P) -> Result<Post> {
        // 读取文件内容
        let content = fs::read_to_string(&path).await?;

        // 从文件路径中提取文件名（不含.md后缀）作为文章URL
        let url = path
            .as_ref()
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("")
            .to_string();

        // 使用 "---" 分隔符将 Markdown 内容分为 Front Matter 和正文
        let mut parts = content.splitn(3, "---");

        // 获取并解析 Front Matter（YAML 格式）
        let front_matter_yaml = parts.nth(1).unwrap_or("");
        let front_matter: FrontMatter = serde_yaml::from_str(front_matter_yaml)?;

        // 获取文章正文内容
        let body = parts.nth(0).unwrap_or("");

        // 构造并返回 Post 对象
        Ok(Post {
            front_matter,
            content: body.to_string(),
            url,
        })
    }

    /// 从文件系统加载所有文章
    /// 扫描 post 目录下的所有 .md 文件，并按日期降序排序
    async fn load_posts_from_fs(&self) -> Result<Vec<Post>> {
        let mut posts = Vec::new();
        // 异步读取 post 目录
        let mut entries = fs::read_dir("post").await?;

        // 遍历目录中的所有条目
        while let Some(entry) = entries.next_entry().await? {
            // 只处理 .md 后缀的文件
            if entry.path().extension().and_then(|s| s.to_str()) == Some("md") {
                let post = self.load_post(entry.path()).await?;
                posts.push(post);
            }
        }
        // 按发布日期降序排序
        posts.sort_by(|a, b| b.front_matter.date.cmp(&a.front_matter.date));
        Ok(posts)
    }

    /// 获取分页的文章归档
    ///
    /// # 参数
    /// * `page` - 页码，从1开始计数
    /// * `per_page` - 每页显示的文章数量
    ///
    /// # 返回
    /// * `Result<(Vec<Archive>, usize)>` - 返回一个元组：
    ///   - 第一个元素是按年份分组的文章归档列表
    ///   - 第二个元素是总条数
    /// # 错误处理
    /// * 如果文章加载失败，将返回错误
    async fn get_paginated_archives(
        &self,
        page: usize,
        per_page: usize,
    ) -> Result<(Vec<Archive>, usize)> {
        // 加载所有文章
        let posts = self
            .load_all_posts()
            .await?
            .into_iter()
            .filter(|p| p.front_matter.draft == false);

        // 使用count()获取过滤后的数量
        let len = posts.clone().count();
        // 获取当前页的文章
        let start = (page - 1) * per_page;
        let page_posts = posts.skip(start).take(per_page);

        // 将文章按年份分组
        let mut archives: Vec<Archive> = Vec::new();

        // 遍历当前页的文章，构建归档结构
        for post in page_posts {
            let year = post.front_matter.date.year() as u32;
            let date = post.front_matter.date.format("%m-%d").to_string();

            let archive_post = ArchivePost {
                date,
                title: post.front_matter.title,
                url: post.url,
            };

            // 尝试将文章添加到已存在的年份分组中
            if let Some(last_archive) = archives.last_mut() {
                if last_archive.year == year {
                    last_archive.posts.push(archive_post);
                    continue;
                }
            }

            // 如果没有匹配的年份，创建新的归档分组
            archives.push(Archive {
                year,
                posts: vec![archive_post],
            });
        }

        Ok((archives, len))
    }
}
