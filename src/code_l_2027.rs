struct Solution;

impl Solution{
    // https://leetcode.cn/problems/minimum-moves-to-convert-string/solutions/
    #[allow(dead_code)]
    pub fn minimum_moves(s: String) -> i32 {
        let (mut covered, mut ans) = (-1, 0);
        s.chars().enumerate().for_each(|(i, c)| {
            if c == 'X' && i as i32 > covered {
                ans += 1;
                covered = i as i32 + 2;
            }
        });
        ans
    }
}

#[cfg(test)]
mod test{
    use super::Solution;

    #[test]
    fn test() {
        let s = "XXX".to_string();
        let ans = Solution::minimum_moves(s);
        assert_eq!(1, ans);
    }
}