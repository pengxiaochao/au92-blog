// 导入所需的模块和类型
use crate::{error::AppError, routes::AppState};
use axum::{
    extract::{Path, State},
    response::Html,
};

/// 处理获取文章归档的请求
/// 参数:
/// - state: 应用程序状态，包含共享的服务实例
/// - page: 可选的页码参数，通过URL路径传入
/// 返回:
/// - Json包装的Archive向量，包含归档列表数据
pub async fn archive_posts(
    State(state): State<AppState>,
    page: Option<Path<usize>>,
) -> Result<Html<String>, AppError> {
    // 从应用状态中获取文章服务实例
    let post_service = &state.post_service;
    // 设置每页显示的记录数
    let per_page = 20;
    // 获取页码，如果未提供则默认为第1页
    let page = page.map(|p| p.0).unwrap_or(1);
    let html = post_service.render_archives(page, per_page).await?;
    Ok(Html(html))
}
