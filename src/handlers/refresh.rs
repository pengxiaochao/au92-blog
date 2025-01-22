// 导入所需的模块和类型
use crate::routes::AppState;
use axum::{extract::State, response::IntoResponse};

/// 刷新所有文章数据的处理函数
/// 参数:
/// - state: 应用程序状态，包含文章服务实例
/// 返回:
/// - 刷新操作的结果信息字符串
pub async fn refresh_posts(State(state): State<AppState>) -> impl IntoResponse {
    // 调用文章服务的刷新方法，并返回相应的成功或失败消息
    match state.post_service.refresh().await {
        Ok(_) => "Posts refreshed successfully".to_string(),
        Err(e) => format!("Failed to refresh posts: {}", e),
    }
}
