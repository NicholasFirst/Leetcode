struct Solution;

impl Solution {
    // https://leetcode.cn/problems/minimum-length-of-string-after-deleting-similar-ends/
    #[allow(dead_code)]
    pub fn minimum_length(s: String) -> i32 {
        let s = s.as_bytes();
        let (mut fi, mut la) = (0, s.len() - 1);
        while fi < la && s[fi] == s[la] {
            let c = s[fi];
            while fi < la && s[fi] == c {
                fi += 1;
            }
            if fi == la{
                return 0;
            }
            while fi < la && s[la] == c {
                la -= 1;
            }
        }
        (la - fi + 1) as i32
    }
}

#[cfg(test)]
mod test{
    use super::Solution;

    #[test]
    fn test() {
        let s = "ca".to_string();
        let ans = Solution::minimum_length(s);
        assert_eq!(ans, 2);
    }



    #[test]
    fn test1() {
        let s = "abccba".to_string();
        let ans = Solution::minimum_length(s);
        assert_eq!(ans, 0);
    }
}