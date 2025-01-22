use serde::{Deserialize, Serialize};

/// 文章归档结构体
/// 
/// # 功能说明
/// - 按年份组织文章列表
/// - 提供年度文章的集合视图
/// 
/// # 字段说明
/// * `year` - 归档年份
/// * `posts` - 该年份下的所有文章列表
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Archive {
    pub year: u32,
    pub posts: Vec<ArchivePost>,
}

/// 归档文章结构体
/// 
/// # 功能说明
/// - 包含归档列表中需要显示的文章基本信息
/// 
/// # 字段说明
/// * `date` - 文章发布日期(MM-DD格式)
/// * `title` - 文章标题
/// * `url` - 文章访问链接
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ArchivePost {
    pub date: String,
    pub title: String,
    pub url: String,
}
