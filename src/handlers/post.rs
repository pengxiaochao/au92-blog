// 导入所需的模块和类型
use crate::{error::AppError, routes::AppState};
use anyhow::Result;
use axum::{
    extract::{Path, State},
    response::Html,
};

/// 处理文章详情页面的请求
/// 
/// # 功能说明
/// - 接收文章URL作为参数
/// - 调用PostService查找并渲染对应的文章
/// - 如果文章存在，返回HTML格式的文章内容
/// - 如果文章不存在，重定向到404错误页面
///
/// # 参数
/// - state: 包含应用共享状态的State包装器，主要用于访问PostService
/// - url: 从URL路径中提取的文章标识符
///
/// # 返回值
/// - 成功：返回HTML格式的文章内容或重定向响应
/// - 失败：返回AppError错误类型
pub async fn post_detail(
    State(state): State<AppState>,
    Path(url): Path<String>,
) -> Result<Html<String>, AppError> {
    let html = state.post_service.render_post(&url).await?;
    Ok(Html(html))
}

/// 处理博客首页的渲染请求
/// 
/// # 功能说明
/// - 支持分页显示文章列表
/// - 处理首页和分页页面的渲染
/// - 集成文章列表和分页信息
///
/// # 参数
/// - state: 应用程序状态，包含文章服务等共享资源
/// - page: 可选的页码参数，通过URL路径获取
///   - 如果没有提供页码，默认显示第1页
///   - 页码从1开始计数
///
/// # 返回值
/// - 成功：返回HTML格式的首页内容
/// - 失败：返回AppError错误类型
///
/// # 特殊说明
/// - 使用axum的debug_handler属性，方便调试
#[axum::debug_handler]
pub async fn render_index(
    State(state): State<AppState>,
    page: Option<Path<usize>>,
) -> Result<Html<String>, AppError> {
    // 处理页码，如果没有提供则默认为第1页
    let page = page.map(|p| p.0).unwrap_or(1);
    // 调用服务层渲染首页内容
    let html = state.post_service.render_index(page).await?;
    // 将渲染结果包装为HTML响应返回
    Ok(Html(html))
}
