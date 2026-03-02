struct Solution;

impl Solution {

    // https://leetcode.cn/problems/minimum-deletions-to-make-string-balanced/
    #[allow(dead_code)]
    pub fn minimum_deletions(s: String) -> i32 {
        s.as_bytes().iter().fold((0, 0), |(ret, b), &ch| if ch == b'a' { (b.min(ret + 1), b) } else { (ret, b + 1) }).0
   }
}

#[cfg(test)]
mod test{
    use super::Solution;

    #[test]
    fn test() {
        let s = "aababbab".to_string();
        let ans = Solution::minimum_deletions(s);
        assert_eq!(2, ans);
    }
    
    #[test]
    fn test1() {
        let s = "bbaaaaabb".to_string();
        let ans = Solution::minimum_deletions(s);
        assert_eq!(2, ans);
    }
}