struct Solution;

impl Solution {   
    // https://leetcode.cn/problems/divide-array-into-arrays-with-max-difference/?envType=daily-question&envId=2025-06-18
    #[allow(dead_code)]
    pub fn divide_array(mut nums: Vec<i32>, k: i32) -> Vec<Vec<i32>> {
        nums.sort();
        let mut res = Vec::new();
        for i in (0..nums.len()).step_by(3) {
            if nums[i + 2] - nums[i] > k {
                return vec![];
            }
            res.push(vec![nums[i], nums[i + 1], nums[i + 2]]);
        }
        res
    }
}

#[cfg(test)]
mod tests {
    
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::divide_array(vec![1,3,4,8,7,9,3,5,1], 2), vec![vec![1,1,3],vec![3,4,5],vec![7,8,9]]);
        assert_eq!(Solution::divide_array(vec![2,4,2,2,5,2], 2), vec![] as Vec<Vec<i32>>);
    }
}