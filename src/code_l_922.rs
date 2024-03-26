use std::{mem, ptr};

struct Solution;

impl Solution {
    #[allow(dead_code)]
    // https://leetcode.cn/problems/sort-array-by-parity-ii/
    pub fn sort_array_by_parity_ii(mut nums: Vec<i32>) -> Vec<i32> {
        //双指针倍内存
        // let mut i = 0;
        // let mut j = 1;
        // let mut new_nums = nums.clone();
        // for v in nums {
        //     if v % 2 == 0 {
        //         new_nums[i] = v;
        //         i += 2;
        //     }else {
        //         new_nums[j] = v;
        //         j += 2;
        //     }
        // }
        // new_nums
        //双指针原始内存
        let mut nums = nums;
        let mut i = 0;
        let mut j = 1;
        while i < nums.len() && j < nums.len() {
            while i < nums.len() && nums[i] & 1 == 0 {
                i += 2;
            }
            while j < nums.len() && nums[j] & 1 == 1 {
                j += 2;
            }
            if i < nums.len() && j < nums.len() {
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
    fn test1() {
        let nums = vec![4, 2, 5, 7];
        let res = Solution::sort_array_by_parity_ii(nums);
        let mut ans = true;
        for (i, v) in res.iter().enumerate() {
            if i % 2 == 0 && v % 2 != 0 {
                ans = false;
                break;
            }
        }
        dbg!(res);
        assert!(ans);
    }
}