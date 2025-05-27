use anyhow::Result;
use axum::http::{header, HeaderValue, StatusCode};
use axum::response::Response;
use axum::{extract::Request, middleware::Next};
use tokio::time::Instant;
use tracing::info;

pub async fn logging(req: Request, next: Next) -> Result<Response, StatusCode> {
    let start = Instant::now();
    let method = req.method().to_string();
    let headers = req.headers().clone(); // 克隆请求头，以便后续使用
    // 提取用户UA信息，如果不存在则使用默认值"unknown"
    let user_agent = headers
        .get(header::USER_AGENT)
        .and_then(|v| v.to_str().ok())
        .unwrap_or("unknown")
        .to_string();
    let ip = headers
        .get("X-Real-IP")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("0.0.0.0")
        .to_string();
    let uri = req.uri().to_string();
    let mut response = next.run(req).await;
    let duration = start.elapsed();
    let ms = duration.as_secs_f64() * 1000.0;
    info!(
        "Request: [{}] [{}] UA:[{}] IP:[{}], Response status: {}, duration time:{}ms",
        method,
        uri,
        user_agent,
        ip,
        response.status(),
        ms
    );
    response.headers_mut().insert(
        "x-response-time",
        HeaderValue::from_str(&format!("{:.2}ms", ms))
            .unwrap_or_else(|_| HeaderValue::from_static("0ms")),
    );
    Ok(response)
}
