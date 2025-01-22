use axum::http::{HeaderValue, StatusCode};
use axum::response::Response;
use axum::{extract::Request, middleware::Next};
use anyhow::Result;
use tokio::time::Instant;

pub async fn logging(req: Request, next: Next) -> Result<Response, StatusCode> {
    let start = Instant::now();
    let method = req.method().to_string();
    let uri = req.uri().to_string();
    let mut response = next.run(req).await;
    let duration = start.elapsed();
    let ms = duration.as_secs_f64() * 1000.0;
    println!(
        "Request: {} {}, Response status: {}, duration time:{}ms",
        method,
        uri,
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
