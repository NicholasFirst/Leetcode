struct Solution;

impl Solution {
    // https://leetcode.cn/problems/count-pairs-that-form-a-complete-day-i/?envType=daily-question&envId=2024-10-22
    #[allow(dead_code)]
    pub fn count_complete_day_pairs(hours: Vec<i32>) -> i32 {
        let mut res = 0;
        for i in 0..hours.len() {
            for j in i + 1..hours.len() {
                if (hours[i] + hours[j]) % 24 == 0 {
                    res += 1;
                }
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_complete_day_pairs() {
        assert_eq!(Solution::count_complete_day_pairs(vec![12,12,30,24,24]), 2);
        assert_eq!(Solution::count_complete_day_pairs(vec![72,48,24,3]), 3);

    }
}