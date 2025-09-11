
struct Solution;

impl Solution{
    
    #[allow(dead_code)]
    #[allow(unused)]
    pub fn max_score(s: String) -> i32 {
        let mut right1 = s.bytes().filter(|&c| c == b'1').count() as i32;
        let mut ans = 0;
        let mut left0 = 0;
        let s = s.as_bytes();
        for &c in &s[..s.len() - 1] { // 移动分割线
            if c == b'0' {
                left0 += 1;
            } else {
                right1 -= 1;
            }
            ans = ans.max(left0 + right1);
        }
        ans
    }
}

#[cfg(test)]
mod test{
    use super::Solution;

    #[test]
    fn test() {
        let s = "011101".to_string();
        let res = Solution::max_score(s);
        assert_eq!(5, res);
    }
}