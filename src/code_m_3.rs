struct Solution;

impl Solution{

    // https://leetcode.cn/problems/longest-substring-without-repeating-characters/description/
    #[allow(dead_code)]
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut last = [-1; 128];
        let mut left = -1;
        let mut ans = 0;
        for (i, v) in s.chars().enumerate() {
            left = left.max(last[v as usize]);
            last[v as usize] = i as i32;
            ans = ans.max(i as i32 - left);
        }
        ans
    }
}

#[cfg(test)]
mod test{
    use super::Solution;

    #[test]
    fn test() {
        let s = "pwwkew".to_string();
        let ans = Solution::length_of_longest_substring(s);
        assert_eq!(3, ans);
    }

    #[test]
    fn test1() {
        let s = "pw!@#wk)(0*^ew".to_string();
        let ans = Solution::length_of_longest_substring(s);
        assert_eq!(11, ans);
    }
}