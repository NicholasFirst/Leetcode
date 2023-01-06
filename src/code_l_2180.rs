struct Solution;

impl Solution {

    // https://leetcode.cn/problems/count-integers-with-even-digit-sum/solutions/?languageTags=rust
    #[allow(dead_code)]
    pub fn count_even(num: i32) -> i32 {
        let sum = num % 10 + (num / 10) % 10 + (num / 100) % 10 + (num / 1000) % 10;
        (num - (sum & 1)) / 2
    }   
}

#[cfg(test)]
mod test{
    use super::Solution;

    #[test]
    fn test() {
        let num = 30;
        let res = Solution::count_even(num);
        assert_eq!(res, 14);
    }
}