use anyhow::Result;
use scraper::{Html, Selector};

/// 移除HTML标签
/// 使用scraper库解析HTML并提取纯文本内容
/// 参数:
/// - html: HTML格式的字符串
pub fn strip_html_tags(html: &str) -> Result<String> {
    // 解析HTML文档
    let document = Html::parse_document(html);

    // 使用通配符选择器获取所有文本内容
    let selector = Selector::parse("*").unwrap();

    // 提取并合并所有文本节点
    let str = document
        .select(&selector)
        .map(|element| element.text().collect::<Vec<_>>().join(" "))
        .collect::<Vec<_>>()
        .join(" ");
    Ok(str)
}

/// 转义所有html 标签，防止XSS攻击
/// 参数:
/// - html: HTML格式的字符串
/// 返回:
/// - 转义后的HTML字符串
pub fn escape_html(html: &str) -> String {
    let mut result = String::new();
    for c in html.chars() {
        match c {
            '<' => result.push_str("&lt;"),
            '>' => result.push_str("&gt;"),
            '&' => result.push_str("&amp;"),
            '"' => result.push_str("&quot;"),
            '/' => result.push_str("&#x2F;"),
            '\'' => result.push_str("&#x27;"),
            _ => result.push(c),
        }
    }
    result
}
