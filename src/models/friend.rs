use serde::{Deserialize, Serialize};

///
/// 友情连接
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FriendLink {
    pub name: String, // 显示名称
    pub url: String,  // 链接地址
    pub avatar: String, // 头像地址
    pub desc: String, // 描述信息
}