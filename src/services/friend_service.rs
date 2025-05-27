use std::{fs::File, io::BufReader, path::Path, sync::Arc};
use anyhow::Result;
use tera::Context;
use crate::models::FriendLink;
use super::TemplateService;

/// 友情连接服务
#[derive(Clone, Debug)]
pub struct FriendLinkService {
    template_service: Arc<TemplateService>,
}
impl FriendLinkService {
    /// 创建新的FriendLinkService实例
    ///
    /// # 参数
    /// * `template_service` - 传入的模板服务实例
    pub fn new(template_service: Arc<TemplateService>) -> Self {
        Self {
            template_service,
        }
    }

    /// 渲染友情连接页面
    /// 
    /// # 返回
    /// * `Result<String>` - 渲染后的HTML字符串
    pub async fn render_friend_links(&self) -> Result<String> {
        let mut context = Context::new();
        let friend_links = self.get_friend_links().await;
        context.insert("friends", &friend_links);
        self.template_service.render("friends.html.tera", &context)
    }
    /// 从/static/friends.yaml文件加载友情连接
    /// 
    /// # 返回
    /// * `Vec<FriendLink>` - 友情连接列表
    async fn get_friend_links(&self) -> Vec<FriendLink> {
        let file_path = Path::new("static/friends.yaml");
        if !file_path.exists() {
            return vec![];
        }
        let file = File::open(file_path).unwrap();
        let reader = BufReader::new(file);
        let friend_links: Vec<FriendLink> = serde_yaml::from_reader(reader).unwrap();
        friend_links
    }
}
