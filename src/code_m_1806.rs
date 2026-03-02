struct Solution;

impl Solution {
    
    // https://leetcode.cn/problems/minimum-number-of-operations-to-reinitialize-a-permutation/description/
    #[allow(dead_code)]
    pub fn reinitialize_permutation(n: i32) -> i32 {
        if n == 2 {
            return 1;
        }
        let mut step = 1;
        let mut pow2 = 2;
        while pow2 != 1 {
            step += 1;
            pow2 = pow2 * 2 % (n - 1);
        }
        step
    }
}

#[cfg(test)]
mod test{
    use super::Solution;

    #[test]
    fn test(){
        let n = 4;
        let res = Solution::reinitialize_permutation(n);
        assert_eq!(res, 2);
    }
}