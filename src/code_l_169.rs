struct Solution;

impl Solution {
    // 投票算法
    #[allow(dead_code)]
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut count = 0;
        let mut candidate = 0;
        for num in nums {
            if count == 0 {
                candidate = num;
            }
            count += if num == candidate { 1 } else { -1 };
        }
        candidate
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::majority_element(vec![2, 3, 3]), 3);
        assert_eq!(Solution::majority_element(vec![6, 6, 6, 7, 7]), 6);
        assert_eq!(Solution::majority_element(vec![2, 2, 1, 1, 1, 2, 2]), 2);
    }
}