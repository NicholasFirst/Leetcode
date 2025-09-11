struct Solution;

impl Solution {
    // https://leetcode.cn/problems/missing-number/description/?envType=problem-list-v2&envId=array
    #[allow(dead_code)]
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        let mut count = 0i32;
        for (index, iter) in nums.iter().enumerate() {
            count += index as i32 + 1;
            count -= iter;
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::missing_number(vec![3,0,1]), 2);
        assert_eq!(Solution::missing_number(vec![0,1]), 2);
        assert_eq!(Solution::missing_number(vec![9,6,4,2,3,5,7,0,1]), 8);
    }
}