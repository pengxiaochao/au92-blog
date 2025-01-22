use serde::{Deserialize, Serialize};

/// 分类统计结构体
/// 
/// # 功能说明
/// - 记录文章分类及其对应的文章数量
/// 
/// # 字段说明
/// * `name` - 分类名称
/// * `count` - 该分类下的文章数量
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CategoryCount {
    pub name: String,
    pub count: usize,
}