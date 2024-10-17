struct Solution;

impl Solution {
    // https://leetcode.cn/problems/minimum-average-of-smallest-and-largest-elements/description/?envType=daily-question&envId=2024-10-16
    #[allow(dead_code)]
    pub fn minimum_average(mut nums: Vec<i32>) -> f64 {
        nums.sort_unstable();
        (0..nums.len() / 2).map(|i| nums[i] + nums[nums.len() - 1 - i])
        .min()
        .unwrap() as f64 / 2.0
    }
}

#[test]
fn test_3194() {
    assert_eq!(Solution::minimum_average(vec![7,8,3,4,15,13,4,1]), 5.5);
    assert_eq!(Solution::minimum_average(vec![1,9,8,3,10,5]), 5.5);
    assert_eq!(Solution::minimum_average(vec![1,2,3,7,8,9]), 5.0);  
}