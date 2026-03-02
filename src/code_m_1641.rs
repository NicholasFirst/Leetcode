struct Solution;

impl Solution {
    // https://leetcode.cn/problems/count-sorted-vowel-strings/
    #[allow(dead_code)]
    pub fn count_vowel_strings(n: i32) -> i32 {
        (n + 1..=n + 4).product::<i32>() / 24
    }
}

#[cfg(test)]
mod test{
    use super::Solution;

    #[test]
    fn test() {
        let n = 2;
        let ans = Solution::count_vowel_strings(n);
        assert_eq!(ans, 15)
    }
}