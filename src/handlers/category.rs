// 导入所需的模块和类型
use crate::{error::AppError, routes::AppState};
use axum::{
    extract::{Path, State},
    response::Html,
};

/// 处理分类首页请求，返回所有分类及其文章数量
/// 参数:
/// - state: 应用程序状态，包含分类服务实例
/// 返回:
/// - Json包装的CategoryCount向量，包含分类统计信息
pub async fn categories_index(State(state): State<AppState>) -> Result<Html<String>, AppError> {
    // 调用分类服务获取所有分类信息
    let html = state.category_service.render_category_page().await?;
    Ok(Html(html))
}

/// 获取指定分类的所有文章列表
/// 参数:
/// - category: 通过URL路径获取的分类名称
/// - state: 应用程序状态，包含分类服务实例
/// 返回:
/// - Json包装的Post向量，包含该分类下的所有文章
pub async fn category_posts(
    Path(category): Path<String>,
    State(state): State<AppState>,
) -> Result<Html<String>, AppError> {
    // 调用分类服务获取指定分类的文章列表
    let html = state
        .category_service
        .render_posts_by_category(category, 1)
        .await?;
    Ok(Html(html))
}

/// 获取指定分类的所有文章列表
/// 参数:
/// - category: 通过URL路径获取的分类名称
/// - page: 分页页码
/// - state: 应用程序状态，包含分类服务实例
/// 返回:
/// - Json包装的Post向量，包含该分类下的所有文章
pub async fn category_posts_with_page(
    Path((category, page)): Path<(String, usize)>,
    State(state): State<AppState>,
) -> Result<Html<String>, AppError> {
    // 调用分类服务获取指定分类的文章列表
    let html = state
        .category_service
        .render_posts_by_category(category, page)
        .await?;
    Ok(Html(html))
}
