struct Solution;

impl Solution {

    // https://leetcode.cn/problems/maximum-score-from-removing-stones/
    #[allow(dead_code)]
    pub fn maximum_score(a: i32, b: i32, c: i32) -> i32 {
        let sum = a + b + c;
        let max_val = a.max(b).max(c);
        (sum - max_val).min(sum / 2)
    }
}

#[cfg(test)]
mod test{
    use super::Solution;

    #[test]
    fn test() {
        let (a, b, c) = (2,4,6);
        let res = Solution::maximum_score(a, b, c);
        assert_eq!(6, res);
    }

    #[test]
    fn test1() {
        let (a, b, c) = (4,4,6);
        let res = Solution::maximum_score(a, b, c);
        assert_eq!(7, res);
    }
}