struct Solution;

use std::collections::HashMap;

impl Solution {

    
    // https://leetcode.cn/problems/maximum-number-of-balls-in-a-box/
    #[allow(dead_code)]
    pub fn count_balls(low_limit: i32, high_limit: i32) -> i32 {

        let mut table = HashMap::new();

        for it in low_limit..=high_limit {
            let total: i32 = it.to_string().chars().map(|c| c.to_string().parse::<i32>().unwrap()).sum();
            let ent = table.entry(total).or_insert(0);
            *ent += 1;
        }
        let mut ans = 0;
        for (_, y) in table.iter() {
            if &ans < y {
                ans = *y;
            }
        }
        ans
    }
}

#[cfg(test)]
mod test{
    use super::Solution;

    #[test]
    fn test() {
        let low_limit = 1;
        let high_limit = 10;
        let ans = Solution::count_balls(low_limit, high_limit);
        assert_eq!(ans, 2)
    }
}