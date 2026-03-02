#[allow(dead_code)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut j = 0usize;
        for i in 0..nums.len() {
            if nums[i] == 0 {
                continue;
            }else {
                nums[j] = nums[i];
                if j != i {
                    nums[i] = 0;
                }
                j += 1;
            } 
        }
    }
}

#[cfg(test)]
mod test{

    use super::*;

    #[test]
    fn test_283() {
        let mut nums = vec![0,1,0,3,12];
        Solution::move_zeroes(&mut nums);
        assert_eq!(vec![1,3,12,0,0], nums);
    }
}