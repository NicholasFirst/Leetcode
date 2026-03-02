struct Solution;

impl Solution {

    #[allow(dead_code)]
    pub fn max_div_score(nums: Vec<i32>, divisors: Vec<i32>) -> i32 {
        let mut max_count = 0;
        let mut result = i32::MAX;
        
        for &divisor in &divisors {
            let count = nums.iter().filter(|&&num| num % divisor == 0).count() as i32;
            if count > max_count || (count == max_count && divisor < result) {
                max_count = count;
                result = divisor;
            }
        }
        
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::max_div_score(vec![20,14,21,10], vec![5,7,5]), 5);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::max_div_score(vec![12], vec![10,16]), 10);
    }
}