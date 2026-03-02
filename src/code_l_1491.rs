struct Solution;


impl Solution {

    #[allow(dead_code)]
    // https://leetcode.cn/problems/average-salary-excluding-the-minimum-and-maximum-salary/?envType=daily-question&envId=2024-05-03
    pub fn average(mut salary: Vec<i32>) -> f64 {
        salary.sort_unstable();
        let mut sum = 0;
        for i in 1..salary.len() - 1 {
            sum += salary[i];
        }
        sum as f64 / (salary.len() - 2) as f64
        
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = Solution::average(vec![4000,3000,1000,2000]);
        assert_eq!(result, 2500.0);
    }
}