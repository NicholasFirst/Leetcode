struct Solution;

impl Solution {
    // https://leetcode.cn/problems/minimize-string-length/description/?envType=daily-question&envId=2025-03-28
    #[allow(dead_code)]
    pub fn minimized_string_length(s: String) -> i32 {
        let mut mask = 0u32;
        for c in s.chars() {
            mask |= 1 << (c as u8 - 'a' as u8);
        }
        mask.count_ones() as i32
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::minimized_string_length("aaabc".to_string()), 3);
        assert_eq!(Solution::minimized_string_length("aaabbaaa".to_string()), 2);
        assert_eq!(Solution::minimized_string_length("aaaa".to_string()), 1);
        assert_eq!(Solution::minimized_string_length("abcbad".to_string()), 4);
    }
}