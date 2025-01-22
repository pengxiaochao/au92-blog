// 导入所需的模块和类型
use crate::routes::AppState;
use axum::extract::State;
use axum::http::Response;
use axum::response::IntoResponse;

/// 生成并返回网站的RSS订阅源
/// 参数:
/// - state: 应用程序状态，包含RSS服务实例
/// 返回:
/// - 包含XML格式的RSS内容的HTTP响应
pub async fn rss_feed(State(state): State<AppState>) -> impl IntoResponse {
    // 调用RSS服务生成订阅源XML内容
    let xml = state.rss_service.generate_feed_xml().await.unwrap();
    
    // 构建HTTP响应，设置Content-Type为application/xml
    Response::builder()
        .header("Content-Type", "application/xml")
        .body(xml)
        .unwrap()
}
