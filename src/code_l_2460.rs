#[allow(dead_code)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn apply_operations(mut nums: Vec<i32>) -> Vec<i32> {
        let mut j = 0;
        for i in 0..nums.len() {
            if i + 1 < nums.len() && nums[i] == nums[i + 1] {
                nums[i] *= 2;
                nums[i + 1] = 0;
            }
            if nums[i] != 0 {
                nums.swap(i, j);
                j += 1;
            }
        }
        nums
    }
}

#[cfg(test)]
mod test{
    use super::Solution;

    #[test]
    fn test() {
        let nums = vec![1,2,2,1,1,0];
        let res = Solution::apply_operations(nums);
        assert_eq!(vec![1,4,2,0,0,0], res);

        let muns_1 = vec![0,1];
        let res = Solution::apply_operations(muns_1);
        assert_eq!(vec![1, 0], res);
    }
}