struct Solution;

impl Solution {
    // https://leetcode.cn/problems/subarray-sum-equals-k/description/
    #[allow(dead_code)]
    pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
        let mut sums = std::collections::HashMap::new();
        let (mut res, mut sum) = (0, 0);
        sums.entry(0).or_insert(1);
        for n in nums {
            sum += n;
            res += sums.get(&(sum-k)).unwrap_or(&0);
            *sums.entry(sum).or_default() += 1;
        }
        res
    }
}

#[cfg(test)]
mod test{
    use super::Solution;

    #[test]
    fn test() {
        let nums = vec![1,1,1];
        let k = 2;
        let res = Solution::subarray_sum(nums, k);
        assert_eq!(res, 2);
    }

    #[test]
    fn test1() {
        let nums = vec![1,2,3];
        let k = 3;
        let res = Solution::subarray_sum(nums, k);
        assert_eq!(res, 2);
    }

    #[test]
    fn test2() {
        let nums = vec![-1,-1,1];
        let k = 0;
        let res = Solution::subarray_sum(nums, k);
        assert_eq!(res, 1);
    }
}