// 导入所需的模块和类型
use crate::{error::AppError, routes::AppState};
use axum::{extract::{State, Multipart}, response::Html};

pub async fn upload_file(State(state): State<AppState>, multipart: Multipart) -> Result<Html<String>, AppError> {
    // 调用上传服务处理文件上传并返回结果
    let html = state.upload_service.upload(multipart).await?;
    Ok(Html(html))
}
