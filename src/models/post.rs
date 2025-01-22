// 导入所需的外部模块和类型
use crate::utils::html::strip_html_tags; // 导入HTML标签清理工具
use crate::utils::{date_format, pinyin}; // 导入日期格式化和拼音转换工具
use chrono::{DateTime, FixedOffset}; // 导入时间处理相关类型
use pulldown_cmark::{html, CowStr, Event, HeadingLevel, Options, Parser, Tag, TagEnd}; // 导入Markdown解析器
use pulldown_cmark_toc::TableOfContents; // 导入目录生成工具
use serde::{Deserialize, Serialize}; // 导入序列化和反序列化trait

/// 文章头部信息（Front Matter）结构体
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FrontMatter {
    pub title: String, // 文章标题
    #[serde(with = "date_format")]
    pub date: DateTime<FixedOffset>, // 文章发布日期，使用自定义日期格式
    #[serde(default = "default_draft")]
    pub draft: bool, // 是否为草稿，默认为false
    #[serde(default)]
    pub categories: Option<Vec<String>>, // 可选的文章分类列表
    #[serde(default)]
    pub tags: Option<Vec<String>>, // 可选的文章标签列表
}

// 定义draft字段的默认值函数
fn default_draft() -> bool {
    false
}

/// 完整的文章结构体
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Post {
    pub front_matter: FrontMatter, // 文章的元数据信息
    pub content: String,           // 文章的主体内容
    pub url: String,               // 文章的URL地址
}

impl Post {
    /// 统计文章中包含的汉字数量
    ///
    /// 通过遍历文章内容的每个字符，判断是否为汉字（Unicode 范围：\u4e00-\u9fff）来进行计数。
    ///
    /// # 返回值
    ///
    /// * `usize` - 文章中汉字的总数
    pub fn count_chinese_chars(&self) -> usize {
        // 使用迭代器过滤出汉字并计数
        self.content.chars().filter(|c| c.is_chinese()).count()
    }

    /// 计算阅读文章所需的预估时间
    ///
    /// 基于文章的汉字数量和给定的阅读速度，计算出预计阅读时间。
    /// 如果计算结果小于1分钟，则返回1分钟作为最小阅读时间。
    ///
    /// # 参数
    ///
    /// * `speed` - 阅读速度，表示每分钟可以阅读的汉字数量
    ///
    /// # 返回值
    ///
    /// * `u16` - 预计阅读时间，单位为分钟
    pub fn read_time(&self, speed: u16) -> u16 {
        let count = self.count_chinese_chars() as u16; // 获取汉字总数
        let time = count / speed; // 计算阅读时间
        if time == 0 {
            1 // 如果计算结果为0，返回1分钟
        } else {
            time // 否则返回计算得到的时间
        }
    }

    /// 生成文章的描述摘要
    ///
    /// 将文章的 Markdown 内容转换为纯文本，并截取指定长度作为描述。
    /// 处理流程：
    /// 1. Markdown 转换为 HTML
    /// 2. 清除 HTML 标签
    /// 3. 清理文本（去除多余空白、合并行）
    /// 4. 截取指定长度
    ///
    /// # 参数
    ///
    /// * `count` - 需要截取的字符数量
    ///
    /// # 返回值
    ///
    /// * `String` - 处理后的文章描述。如果处理过程中出现错误，则返回空字符串
    pub fn generate_description(&self, count: usize) -> String {
        // 创建Markdown解析器实例
        let parser = Parser::new(self.content.as_str());
        let mut html_output = String::new();
        // 将Markdown内容转换为HTML
        html::push_html(&mut html_output, parser);

        // 尝试清理HTML标签
        match strip_html_tags(html_output.as_str()) {
            Ok(text) => {
                // 清理文本：去除多余空白，合并行
                let cleaned_text = text
                    .lines()
                    .map(|line| line.trim()) // 移除每行首尾空白
                    .filter(|line| !line.is_empty()) // 过滤掉空行
                    .collect::<Vec<_>>()
                    .join(" "); // 用空格连接所有行
                                // 截取指定长度的描述文本
                cleaned_text.chars().take(count).collect()
            }
            Err(_) => String::new(), // 处理失败时返回空字符串
        }
    }

    /// 生成文章的HTML内容，为标题添加ID属性
    ///
    /// 处理流程：
    /// 1. 初始化解析器
    ///    - 不需要特殊的解析选项
    ///    - 创建基本的Markdown解析器实例
    ///
    /// 2. 标题处理策略：
    ///    - 跟踪h2和h3级别的标题
    ///    - 收集标题文本内容
    ///    - 将标题文本转换为拼音作为ID
    ///    - 生成带有ID属性的HTML标题标签
    ///
    /// 3. 事件处理逻辑：
    ///    - 标题开始：记录当前标题级别，准备收集内容
    ///    - 文本内容：当在标题中时，累积文本内容
    ///    - 标题结束：生成带ID的完整标题HTML
    ///
    /// 4. 其他内容保持不变
    ///
    /// # 返回值
    /// * `String` - 转换后的HTML内容
    pub fn generate_html(&self) -> String {
        // 创建基本的解析器选项和解析器实例
        let options = Options::empty();
        let parser = Parser::new_ext(&self.content, options);

        // 用于追踪标题状态的变量
        let mut current_heading_content = String::new(); // 收集当前标题的文本内容
        let mut current_heading_level = None; // 记录当前处理的标题级别

        // 转换事件流，处理标题并添加ID属性
        let parser = parser.filter_map(|event| {
            match event {
                // 处理标题开始标签
                Event::Start(Tag::Heading {
                    level,
                    id: _,
                    classes: _,
                    attrs: _,
                }) if level == HeadingLevel::H2
                    || level == HeadingLevel::H3
                    || level == HeadingLevel::H4 =>
                {
                    // 重置标题收集状态
                    current_heading_content.clear();
                    current_heading_level = Some(level);
                    None // 关键：不输出原始的开始标签
                }
                // 收集标题文本内容
                Event::Text(text) if current_heading_level.is_some() => {
                    current_heading_content.push_str(&text); // 累积标题文本
                    None // 关键：暂存文本，不立即输出
                }
                // 处理标题结束标签
                Event::End(TagEnd::Heading(level)) if current_heading_level == Some(level) => {
                    // 生成拼音ID
                    let id = pinyin::to_pinyin(current_heading_content.as_str()).join("-");
                    // 构造带ID的HTML标题
                    let heading_html = format!(
                        r#"<h{} id="{}">{}</h{}>"#,
                        level as u8, id, current_heading_content, level as u8
                    );

                    // 重置状态并输出处理后的HTML
                    current_heading_level = None;
                    current_heading_content.clear();
                    Some(Event::Html(CowStr::from(heading_html)))
                }
                // 保持其他内容不变
                other => Some(other),
            }
        });

        // 生成最终的HTML文档
        let mut html_output = String::new();
        html::push_html(&mut html_output, parser);
        html_output
    }

    /// 生成文章的目录结构
    ///
    /// 解析文章内容中的标题标记，生成层级化的目录结构。
    /// 每个目录项包含：
    /// - 标题层级（如：1表示h1，2表示h2）
    /// - 标题文本
    /// - 由标题转换的拼音ID（用于锚点链接）
    ///
    /// # 返回值
    ///
    /// * `Vec<(usize, String, String)>` - 目录结构的向量
    ///   - `usize`: 标题层级
    ///   - `String`: 标题文本
    ///   - `String`: 标题对应的拼音ID
    pub fn generate_toc(&self) -> Vec<(usize, String, String)> {
        let mut toc = Vec::new();
        // 创建目录生成器实例
        let result = TableOfContents::new(&self.content);
        // 遍历所有标题并处理
        result.headings().for_each(|h| {
            let id = pinyin::to_pinyin(h.text().as_str()).join("-"); // 将标题文本转换为拼音作为ID
            toc.push((h.level() as usize, h.text(), id)); // 将标题信息添加到目录中
        });
        // let mark = result.to_cmark();                        // 转换为Markdown格式
        // println!("toc: {:?}", mark);                         // 输出调试信息
        toc // 返回生成的目录
    }
}

/// 定义用于判断字符是否为汉字的特征
///
/// 此特征提供了一个方法来判断字符是否属于汉字范围（Unicode: 4E00-9FFF）
trait ChineseChar {
    /// 判断字符是否为汉字
    ///
    /// # 返回值
    ///
    /// * `bool` - 如果字符是汉字则返回 true，否则返回 false
    fn is_chinese(&self) -> bool;
}

impl ChineseChar for char {
    fn is_chinese(&self) -> bool {
        // 使用Unicode范围判断是否为汉字（基本汉字范围：4E00-9FFF）
        matches!(self, '\u{4e00}'..='\u{9fff}')
    }
}
