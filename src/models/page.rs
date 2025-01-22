use serde::{Deserialize, Serialize};

/// 分页数据结构体，用于处理列表数据的分页信息
/// 封装了分页相关的所有计算逻辑，包括：
/// - 总页数计算
/// - 当前页码处理
/// - 上一页/下一页的自动计算
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Page {
    /// 总页数，根据总条数和每页条数计算得出
    pub count: u16,
    /// 当前页码，从1开始，不能大于总页数
    pub current: u16,
    /// 上一页的页码，如果当前是第一页则为None
    /// 用于分页导航中上一页按钮的显示和跳转
    pub prev: Option<u16>,
    /// 下一页的页码，如果当前是最后一页则为None
    /// 用于分页导航中下一页按钮的显示和跳转
    pub next: Option<u16>,
}

impl Page {
    /// 创建新的分页实例，根据总条数和每页条数自动计算分页信息
    /// 
    /// # 实现说明
    /// 1. 通过总条数和每页条数计算总页数
    /// 2. 确保当前页不超过总页数
    /// 3. 自动计算上一页和下一页
    /// 
    /// # 参数说明
    /// * `total` - 数据总条数，如列表中总共有多少条记录
    /// * `current` - 当前请求的页码，从1开始
    /// * `per_page` - 每页显示的记录数，用于计算总页数
    /// 
    /// # 返回值
    /// 返回配置好的分页实例，包含了所有必要的分页信息
    pub fn new(total: u32, current: u16, per_page: u16) -> Self {
        // 计算总页数：总条数除以每页条数，向上取整
        // 使用 f64 确保除法计算精确，最后转回 u16
        let count = ((total as f64) / (per_page as f64)).ceil() as u16;
        
        // 确保当前页不超过总页数，如果超过则设置为最后一页
        let current = if current > count { count } else { current };
        
        // 计算上一页：当前页大于1时才有上一页
        let prev = if current > 1 { Some(current - 1) } else { None };

        // 计算下一页：当前页小于总页数时才有下一页
        let next = if current < count {
            Some(current + 1)
        } else {
            None
        };

        Self {
            count,
            current,
            prev,
            next,
        }
    }

    /// 通过直接指定总页数来创建分页实例（向后兼容的方法）
    /// 
    /// # 实现说明
    /// 1. 确保当前页在有效范围内
    /// 2. 计算上一页和下一页
    /// 3. 通过总页数估算总条数（假设每页10条）
    /// 
    /// # 参数说明
    /// * `count` - 直接指定的总页数
    /// * `current` - 当前请求的页码
    /// 
    /// # 特殊说明
    /// 这是一个为了保持与旧版本兼容的方法，优先使用 new() 方法
    pub fn from_count(count: u16, current: u16) -> Self {
        // 确保当前页不超过总页数
        let current = if current > count { count } else { current };
        // 计算上一页
        let prev = if current > 1 { Some(current - 1) } else { None };
        // 计算下一页
        let next = if current < count {
            Some(current + 1)
        } else {
            None
        };

        Self {
            count,
            current,
            prev,
            next,
        }
    }
}
