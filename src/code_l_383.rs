struct Solution;

impl Solution {
    #[allow(dead_code)]
    // https://leetcode.cn/problems/ransom-note/?envType=daily-question&envId=2024-01-07
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        let mut cnt = vec![0; 26];
        for c in magazine.chars() {
            cnt[(c as u8 - b'a') as usize] += 1;
        }
        for c in ransom_note.chars() {
            cnt[(c as u8 - b'a') as usize] -= 1;
            if cnt[(c as u8 - b'a') as usize] < 0 {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_construct() {
        let r = Solution::can_construct("a".to_string(), "b".to_string());
        assert_eq!(r, false);
        let r = Solution::can_construct("aa".to_string(), "ab".to_string());
        assert_eq!(r, false);
        let r = Solution::can_construct("aa".to_string(), "aab".to_string());
        assert_eq!(r, true);
    }
}