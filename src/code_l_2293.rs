struct Solution;

impl Solution {

    // https://leetcode.cn/problems/min-max-game/
    #[allow(dead_code)]
    pub fn min_max_game(mut nums: Vec<i32>) -> i32 {
        while nums.len() > 1 {
            nums = nums
                .chunks(2)
                .enumerate()
                .map(|(i, v)| match i % 2 {
                    0 => *v.iter().min().unwrap(),
                    _ => *v.iter().max().unwrap(),
                })
                .collect();
        }
        nums[0]
    }
}


#[cfg(test)]
mod test{
    use super::Solution;

    #[test]
    fn test() {
        let nums = vec![1,3,5,2,4,8,2,2];
        let ans = Solution::min_max_game(nums);
        assert_eq!(ans, 1);
    }
}