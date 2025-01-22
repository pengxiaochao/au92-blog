use axum::{
    http::StatusCode,
    response::{IntoResponse, Redirect, Response},
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Internal server error: {0}")]
    Internal(#[from] anyhow::Error),
    #[error("Not found: {0}")]
    NotFound(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            Self::Internal(err) => {
                tracing::error!("Internal error: {:?}", err);
                (StatusCode::INTERNAL_SERVER_ERROR, format!("Internal Server Error: {}", err))
                    .into_response()
            }
            Self::NotFound(url) => {
                // 对于404错误，重定向到error页面
                Redirect::to(&format!("/error/404.html?url={}", url)).into_response()
            }
        }
    }
}
