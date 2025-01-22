use std::sync::Arc;
use tera::{Tera, Value, try_get_value};
use anyhow::Result;
use crate::{models::Site, utils::date};

/// 模板渲染服务
/// 
/// # 功能描述
/// - 管理和初始化Tera模板引擎
/// - 注册自定义模板过滤器
/// - 提供模板渲染功能
/// 
/// # 字段说明
/// * `tera` - Tera模板引擎实例，使用Arc实现线程安全共享
/// * `site` - 网站全局配置信息
#[derive(Clone,Debug)]
pub struct TemplateService {
    /// Tera模板引擎实例，使用Arc实现线程安全的共享
    tera: Arc<Tera>,
    /// 站点配置信息，包含全局设置和元数据
    site: Site,
}

impl TemplateService {
    /// 创建新的模板服务实例
    /// 
    /// # 功能
    /// - 初始化Tera模板引擎
    /// - 注册自定义过滤器
    /// - 加载站点配置
    /// 
    /// # 返回
    /// * `Result<Self>` - 成功返回服务实例，失败返回错误
    /// 
    /// # 错误
    /// * 当模板目录不存在或模板文件无法解析时返回错误
    pub fn new() -> Result<Self> {
        // 初始化Tera模板引擎，加载templates目录下所有模板
        let mut tera = Tera::new("templates/**/*")?;
        tera.register_filter("format_date", format_date_filter);
        tera.register_filter("nl2p", nl2p_filter);
        // 从环境变量加载站点配置
        let site = Site::from_env();
        
        Ok(Self {
            tera: Arc::new(tera),
            site,
        })
    }

    /// 渲染指定的模板
    /// 
    /// # 参数
    /// * `template_name` - 模板文件名
    /// * `context` - 模板渲染上下文数据
    /// 
    /// # 返回
    /// * `Result<String>` - 渲染后的HTML字符串或错误
    pub fn render(&self, template_name: &str, context: &tera::Context) -> Result<String> {
        let mut context = context.clone();
        // 注入站点配置到模板上下文
        context.insert("site", &self.site);
        Ok(self.tera.render(template_name, &context)?)
    }
}

/// 日期格式化过滤器
/// 
/// # 参数
/// * `value` - 待格式化的时间戳值
/// * `args` - 过滤器参数，包含format参数指定输出格式
/// 
/// # 返回
/// * `tera::Result<Value>` - 格式化后的日期字符串
fn format_date_filter(value: &Value, args: &std::collections::HashMap<String, Value>) -> tera::Result<Value> {
    let format = args.get("format")
        .and_then(|v| v.as_str())
        .unwrap_or("%Y-%m-%d %H:%M:%S");

    match value.as_i64() {
        Some(timestamp) => Ok(Value::String(date::format_timestamp(timestamp, format))),
        None => Ok(Value::String("Invalid date".to_string())),
    }
}

/// 文本转HTML段落过滤器
/// 
/// # 参数
/// * `value` - 待转换的纯文本
/// * `_` - 未使用的过滤器参数
/// 
/// # 返回
/// * `tera::Result<Value>` - 转换后的HTML段落
fn nl2p_filter(value: &Value, _: &std::collections::HashMap<String, Value>) -> tera::Result<Value> {
    let text = try_get_value!("nl2p", "value", String, value);
    Ok(Value::String(text.replace('\n', "</p><p>")))
}