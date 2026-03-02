struct Solution;

impl Solution {
    // https://leetcode.cn/problems/count-subarrays-of-length-three-with-a-condition/?envType=daily-question&envId=2025-04-27
    #[allow(dead_code)]
    pub fn count_subarrays(nums: Vec<i32>) -> i32 {
        let mut ans = 0;
        nums.windows(3).for_each(|x| {
            if (x[0] + x[2]) * 2 == x[1] {
                ans += 1;
            }
        });
        ans
    }
}


#[cfg(test)]
mod tests {
    
    use super::Solution;

    #[test]
    fn test_count_subarrays() {
        assert_eq!(Solution::count_subarrays(vec![1,2,1,4,1]), 1);
        assert_eq!(Solution::count_subarrays(vec![1,1,1]), 0);
    }
}