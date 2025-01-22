// 导入所需的模块和类型
use axum::{
    extract::{Path, State},
    response::Html,
};

use crate::{error::AppError, routes::AppState};

/// 处理标签首页请求，返回所有标签及其文章数量
/// 参数:
/// - state: 应用程序状态，包含标签服务实例
/// 返回:
/// - Json包装的TagCount向量，包含标签统计信息
pub async fn tags_index(State(state): State<AppState>) -> Result<Html<String>, AppError> {
    // 调用标签服务获取所有标签信息并返回
    let html = state.tag_service.render_tag_page().await?;
    Ok(Html(html))
}

/// 处理带页码的标签文章列表
pub async fn tag_posts_with_page(
    State(state): State<AppState>,
    Path((tag, page)): Path<(String, usize)>,
) -> Result<Html<String>, AppError> {
    let html = state.tag_service.render_posts_by_tag(tag, page).await?;
    Ok(Html(html))
}

/// 处理不带页码的标签文章列表
pub async fn tag_posts(
    State(state): State<AppState>,
    Path(tag): Path<String>,
) -> Result<Html<String>, AppError> {
    let html = state.tag_service.render_posts_by_tag(tag, 1).await?;
    Ok(Html(html))
}
