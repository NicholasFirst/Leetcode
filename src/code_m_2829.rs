struct Solution;

impl Solution {
    // https://leetcode.cn/problems/determine-the-minimum-sum-of-a-k-avoiding-array/description/?envType=daily-question&envId=2025-03-26
    #[allow(dead_code)]
    pub fn minimum_sum(n: i32, k: i32) -> i32 {
        let mut sum = 0;
        let mut cnt = 0;
        let mut add = 1;
        let mut p = k;
        while cnt < n {
            if add <= k / 2 {
                sum += add;
                add += 1;
            }else {
                sum += p;
                p += 1;
            }
            cnt += 1;
        }
        sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::minimum_sum(5, 4), 18);
        assert_eq!(Solution::minimum_sum(2, 6), 3);
    }
}