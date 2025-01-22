/// 站点地图URL结构体
/// 表示站点地图中的单个URL条目
#[derive(Debug)]
pub struct SitemapUrl {
    /// URL地址
    pub loc: String,
    /// 最后修改时间，格式符合W3C规范
    pub lastmod: String,
    /// URL优先级，范围0.0-1.0
    pub priority: String,
}

/// 站点地图结构体
/// 包含整个站点的URL列表
#[derive(Debug)]
pub struct Sitemap {
    /// 所有URL条目的集合
    pub urls: Vec<SitemapUrl>,
}
