use crate::{error::AppError, routes::AppState};
use anyhow::Result;
use axum::{extract::State, response::Html};

/// 处理获取友情链接的请求
/// 参数:
/// - state: 应用程序状态，包含共享的服务实例
/// # 返回值
/// - 成功：返回HTML格式的文章内容或重定向响应
/// - 失败：返回AppError错误类型
#[axum::debug_handler]
pub async fn render_friend_links(State(state): State<AppState>) -> Result<Html<String>, AppError> {
    // 调用服务层渲染首页内容
    let html = state.friend_service.render_friend_links().await?;
    // 将渲染结果包装为HTML响应返回
    Ok(Html(html))
}
