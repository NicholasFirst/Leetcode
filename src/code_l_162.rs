struct Solution;

impl Solution {
    pub fn find_peak_element(nums: Vec<i32>) -> i32 {
        let (mut l, mut r) = (0, nums.len() - 1);
        while l < r {
            let mid = l + (r - l >> 1);
            match nums[mid] > nums[mid + 1] {
                true => r = mid,
                false => l = mid + 1,
            }
        }
        l as i32
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_find_peak_element(){
        let nums = vec![1,2,3,1];
        assert_eq!(Solution::find_peak_element(nums), 2);
    }
}