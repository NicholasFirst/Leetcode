struct Solution;

impl Solution {

    // https://leetcode.cn/problems/count-number-of-homogenous-substrings/description/?languageTags=rust
    #[allow(dead_code)]
    pub fn count_homogenous(s: String) -> i32 {
        let s = s.into_bytes();
        let (mut ans, mut i, mut j) = (0, 0, 0);
        while j < s.len() {
            if s[i] == s[j] {
                j += 1;
            } else {
                i = j;
            }
            ans = (ans + j - i) % 1_0000_0000_7;
        }
        ans as i32
    }
}

#[cfg(test)]
mod test{
    use super::Solution;

    #[test]
    fn test() {
        let s = "abbcccaa".to_string();
        let ans = Solution::count_homogenous(s);
        assert_eq!(ans, 13);
    }
} 