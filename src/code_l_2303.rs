struct Solution;

impl Solution {

    // https://leetcode.cn/problems/calculate-amount-paid-in-taxes/
    #[allow(dead_code)]
    pub fn calculate_tax(brackets: Vec<Vec<i32>>, income: i32) -> f64 {
        let mut res = 0f64;
        let mut pre = 0;
        for t in &brackets {
            if pre >= income {
                break;
            }
            res += (std::cmp::min(t[0], income) - pre) as f64 * t[1] as f64;
            pre = t[0];
        }
        return res / 100.0;
    }
}

#[cfg(test)]
mod test{
    use super::Solution;

    #[test]
    fn test() {
        let brackets = vec![vec![3,50],vec![7,10],vec![12,25]]; 
        let income = 10;
        let ans = Solution::calculate_tax(brackets, income);
        assert!((ans - 2.65000).abs() < 0.00001);
    }
}