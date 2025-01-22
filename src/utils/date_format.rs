use chrono::{DateTime, FixedOffset};
use serde::{self, Deserialize, Deserializer, Serializer};

/// 将日期序列化为指定格式的字符串
/// 格式: YYYY-MM-DDThh:mm:ss+zzzz
pub fn serialize<S>(date: &DateTime<FixedOffset>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let s = date.format("%Y-%m-%dT%H:%M:%S%z").to_string();
    serializer.serialize_str(&s)
}

/// 从字符串反序列化为日期对象
/// 支持格式: YYYY-MM-DDThh:mm:ss+zzzz
pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<FixedOffset>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    DateTime::parse_from_str(&s, "%Y-%m-%dT%H:%M:%S%z")
        .map_err(serde::de::Error::custom)
}
