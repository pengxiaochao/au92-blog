use serde::{Deserialize, Serialize};

/// 标签统计信息结构体
/// - name: 标签名称
/// - count: 使用该标签的文章数量
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TagCount {
    pub name: String,
    pub count: usize,
}