struct Solution;

use std::collections::HashMap;

impl Solution {
    // https://leetcode.cn/problems/decode-the-message/
    #[allow(dead_code)]
    pub fn decode_message(key: String, message: String) -> String {
        let mut map = HashMap::from([(' ', ' ')]);
        let mut cur = 97u8;
        key.chars().for_each(|c| {
            if !map.contains_key(&c) {
                map.insert(c, cur as char);
                cur += 1;
            }
        });
        message.chars().map(|c| map.get(&c).unwrap()).collect()
    }
}

#[cfg(test)]
mod test{
    use super::Solution;

    #[test]
    fn test() {
        let key = "the quick brown fox jumps over the lazy dog".to_string();
        let message = "vkbs bs t suepuv".to_string();
        let res = Solution::decode_message(key, message);
        assert_eq!(res, "this is a secret".to_string());
    }
}