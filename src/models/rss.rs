
#[derive(Debug)]
pub struct RssItem {
    pub title: String,
    pub link: String,
    pub pub_date: String,
    pub description: String,
}

#[derive(Debug)]
pub struct RssFeed {
    pub items: Vec<RssItem>,
    pub last_build_date: String,
    pub site_url: String,
    pub site_title: String,
}
