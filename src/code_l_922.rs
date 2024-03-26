struct Solution;

impl Solution {
    #[allow(dead_code)]
    // https://leetcode.cn/problems/sort-array-by-parity-ii/
    pub fn sort_array_by_parity_ii(nums: Vec<i32>) -> Vec<i32> {
        let mut i = 0;
        let mut j = 1;
        let mut new_nums = nums.clone();
        for v in nums {
            if v % 2 == 0 {
                new_nums[i] = v;
                i += 2;
            }else {
                new_nums[j] = v;
                j += 2;
            }
        }
        new_nums
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