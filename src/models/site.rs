use chrono::Datelike;
use serde::{Deserialize, Serialize};
use std::fs;
use toml;

/// 站点配置结构体
/// 用于存储网站的基本配置信息
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Site {
    /// 站点标题
    /// 显示在浏览器标题栏和页面头部
    pub title: String,
    /// 站点URL
    /// 网站的完整访问地址，包括协议和域名
    pub url: String,
    /// RSS Feed中包含的文章数量
    pub rss_count: usize,
    /// RSS Feed中文章描述的最大长度
    pub rss_length: usize,
    /// sitemap url权重
    pub priority: String,
    /// 网站关键词
    pub keywords: String,
    /// 网站描述
    pub description: String,
    /// 作者
    pub author: String,
    /// 站点导航
    pub menus: Vec<Menu>,
    /// 当前年
    pub year: u32,
}

/// 站点导航
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Menu {
    url: String,
    name: String,
    weight: u32,
    identifier: String,
}

impl Site {
    /// 从config.toml中读取配置内容
    fn load_menu_config() -> Vec<Menu> {
        let config_str = fs::read_to_string("config.toml").unwrap_or_else(|_| String::from(""));

        let config: toml::Value =
            toml::from_str(&config_str).unwrap_or_else(|_| toml::Value::Table(toml::Table::new()));

        let binding = Vec::new();
        let menu = config
            .get("menu")
            .and_then(|m| m.get("main"))
            .and_then(|m| m.as_array())
            .unwrap_or(&binding);

        menu.iter()
            .filter_map(|item| {
                if let (Some(url), Some(name), Some(weight), Some(identifier)) = (
                    item.get("url").and_then(|v| v.as_str()),
                    item.get("name").and_then(|v| v.as_str()),
                    item.get("weight").and_then(|v| v.as_integer()),
                    item.get("identifier").and_then(|v| v.as_str()),
                ) {
                    Some(Menu {
                        url: url.to_string(),
                        name: name.to_string(),
                        weight: weight as u32,
                        identifier: identifier.to_string(),
                    })
                } else {
                    None
                }
            })
            .collect()
    }

    /// 从环境变量中读取站点配置
    /// 如果环境变量不存在，则使用默认值
    ///
    /// 环境变量：
    /// - SITE_TITLE: 站点标题
    /// - SITE_URL: 站点URL
    pub fn from_env() -> Self {
        Self {
            title: std::env::var("SITE_TITLE").unwrap_or_else(|_| "Default Title".to_string()),
            url: std::env::var("SITE_URL").unwrap_or_else(|_| "http://localhost:3000".to_string()),
            rss_count: std::env::var("RSS_COUNT")
                .unwrap_or_else(|_| "10".to_string())
                .parse()
                .unwrap_or(10),
            rss_length: std::env::var("RSS_LENGTH")
                .unwrap_or_else(|_| "200".to_string())
                .parse()
                .unwrap_or(200),
            priority: std::env::var("PRIORITY").unwrap_or_else(|_| "0.5".to_string()),
            keywords: std::env::var("KEYWORDS").unwrap_or_else(|_| "".to_string()),
            description: std::env::var("DESCRIPTION").unwrap_or_else(|_| "".to_string()),
            author: std::env::var("AUTHOR").unwrap_or_else(|_| "".to_string()),
            menus: Self::load_menu_config(),
            year: chrono::Local::now().year_ce().1,
        }
    }
}
