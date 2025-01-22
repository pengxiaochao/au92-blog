use pinyin::ToPinyin;
/// 将中文文本转换为拼音
pub fn to_pinyin(text: &str) -> Vec<String> {
    let mut result = Vec::new();

    for c in text.chars() {
        if let Some(pinyin) = c.to_pinyin() {
            // 获取不带声调的拼音
            let pinyin_with_tone = pinyin.plain();
            result.push(pinyin_with_tone.to_string());
        } else {
            // 过滤标点符号
            if c.is_ascii_punctuation() {
                continue;
            }
            // 如果不是汉字，直接加原字符
            result.push(c.to_string());
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chinese_characters() {
        let text = "中国";
        let result = to_pinyin(text);
        assert_eq!(result, vec!["zhong", "guo"]);
    }

    #[test]
    fn test_mixed_characters() {
        let text = "Hello中国123";
        let result = to_pinyin(text);
        assert_eq!(
            result,
            vec!["H", "e", "l", "l", "o", "zhong", "guo", "1", "2", "3"]
        );
    }

    #[test]
    fn test_non_chinese() {
        let text = "Hello123";
        let result = to_pinyin(text);
        assert_eq!(result, vec!["H", "e", "l", "l", "o", "1", "2", "3"]);
    }

    //测试标点符号
    #[test]
    fn test_punctuation() {
        let text = "Hello, 中国!";
        let result = to_pinyin(text);
        assert_eq!(
            result,
            vec!["H", "e", "l", "l", "o",  " ", "zhong", "guo"]
        );
    }
}
