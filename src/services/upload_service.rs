//! 本模块提供文件上传服务，用于处理客户端上传的文件并保存到本地。

use anyhow::Result;
use axum::extract::Multipart;
use std::path::Path;
use tokio::fs;

/// 文件上传服务
///
/// 该服务用于接收并处理客户端上传的文件，
pub struct UploadService {}

impl UploadService {
    /// 创建新的 UploadService 实例
    ///
    /// # 返回
    /// - `Self` - 返回一个新的 UploadService 实例
    ///
    /// # 示例
    /// ```
    /// let service = UploadService::new();
    /// ```
    pub fn new() -> Self {
        Self {}
    }

    /// 处理文件上传请求
    ///
    /// 该方法从 Multipart 表单中提取文件，并使用文件的原始文件名进行保存。
    ///
    /// # 参数
    /// - `multipart` - 上传的 Multipart 数据结构
    ///
    /// # 返回
    /// - `Result<String>` - 成功时返回保存的文件信息，失败时返回错误信息
    ///
    /// # 错误
    /// - 当上传的字段缺失文件名时，返回错误。
    /// - 当文件已存在时，返回文件已存在错误。
    pub async fn upload(&self, mut multipart: Multipart) -> Result<String> {
        // 保存上传的文件数据、原始文件名和自定义保存文件名
        let mut file_bytes = None;
        let mut original_name = None;
        let mut custom_file_name = None;

        while let Some(field) = multipart.next_field().await? {
            if let Some(name) = field.name() {
                match name {
                    "file" => {
                        // 处理上传的文件字段
                        let fname = field
                            .file_name()
                            .ok_or_else(|| anyhow::anyhow!("File name is missing"))?
                            .to_string();
                        let data = field.bytes().await?;
                        file_bytes = Some(data);
                        original_name = Some(fname);
                    },
                    "file_name" => {
                        // 处理自定义文件名字段，读取文本数据
                        let custom = field.text().await?;
                        custom_file_name = Some(custom);
                    },
                    _ => {
                        // 忽略其他字段
                    }
                }
            }
        }

        // 确保上传的文件存在
        let data = file_bytes.ok_or_else(|| anyhow::anyhow!("Missing file field"))?;
        let orig_name = original_name.ok_or_else(|| anyhow::anyhow!("Missing original file name"))?;
        // 如果传入了自定义文件名且非空，则使用自定义文件名，否则使用原始文件名
        let final_name = match custom_file_name {
            Some(ref name) if !name.trim().is_empty() => name.clone(),
            _ => orig_name,
        };

        // 构造保存文件的本地路径
        let file_path = format!("post/{}", final_name);
        // 检查文件是否已存在，防止覆盖
        if Path::new(&file_path).exists() || fs::metadata(&file_path).await.is_ok() {
            return Err(anyhow::anyhow!("File `{}` already exists", final_name));
        }
        let length = data.len();
        // 将文件数据写入本地文件系统
        fs::write(&file_path, data).await?;
        Ok(format!("File `{}` saved, {} bytes", final_name, length))
    }
}
