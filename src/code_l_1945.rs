struct Solution;

impl Solution {
    // https://leetcode.cn/problems/sum-of-digits-of-string-after-convert/
    #[allow(dead_code)]
    pub fn get_lucky(s: String, k: i32) -> i32 {
        (1..k).fold(
            s.bytes()
                .map(|x| (x - 96) as i32)
                .map(|x| x / 10 + x % 10)
                .sum(),
            |x, _| x.to_string().bytes().map(|x| (x - 48) as i32).sum(),
        )
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test() {
        let s = "zbax".to_string();
        let k = 2;
        let ans = Solution::get_lucky(s, k);
        assert_eq!(8, ans);
    }
}
