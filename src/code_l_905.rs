struct Solution;

impl Solution {
    // https://leetcode.cn/problems/sort-array-by-parity/?envType=problem-list-v2&envId=sorting
    #[allow(dead_code)]
    pub fn sort_array_by_parity(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        let mut i = 0;
        let mut j = nums.len() - 1;
        while i < j {
            if nums[i] % 2 == 0 {
                i += 1;
            } else if nums[j] % 2 == 1 {
                j -= 1;
            } else {
                nums.swap(i, j);
            }
        }
        nums
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_905() {
        let nums = vec![3, 1, 2, 4];
        let ans = Solution::sort_array_by_parity(nums);
        let mut flag = false;
        ans.iter().for_each(|x| {
            if flag {
                assert_eq!(*x % 2, 1);
            } else {
                if *x % 2 == 1 {
                    flag = true;
                }else {
                    assert_eq!(*x % 2, 0);
                }
            }
        })
    }
}