// 导入所需的模块和类型
use crate::routes::AppState;
use axum::extract::State;
use axum::http::Response;
use axum::response::IntoResponse;

/// 生成并返回网站的sitemap.xml文件
/// 参数:
/// - state: 应用程序状态，包含站点地图服务实例
/// 返回:
/// - 包含XML内容的HTTP响应，设置正确的Content-Type
pub async fn sitemap_xml(State(state): State<AppState>) -> impl IntoResponse {
    // 调用站点地图服务生成XML内容
    let xml = state.sitemap_service.generate_sitemap_xml().await.unwrap();
    
    // 构建HTTP响应，设置Content-Type为application/xml
    Response::builder()
        .header("Content-Type", "application/xml")
        .body(xml)
        .unwrap()
}
