struct Solution;

use std::collections::HashMap;

impl Solution {

    #[allow(dead_code)]
    #[allow(unused_variables)]
    //https://leetcode.cn/problems/number-of-boomerangs/?envType=daily-question&envId=2024-01-08
    pub fn number_of_boomerangs(points: Vec<Vec<i32>>) -> i32 {
        let mut ans = 0;
        let mut cnt = HashMap::new();
        for p1 in &points {
            cnt.clear();
            for p2 in &points {
                let d2 = (p1[0] - p2[0]) * (p1[0] - p2[0]) + (p1[1] - p2[1]) * (p1[1] - p2[1]);
                let v = cnt.entry(d2).or_insert(0);
                ans += *v * 2;
                *v += 1;
            }
        }
        ans
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_number_of_boomerangs() {
        let result = Solution::number_of_boomerangs(vec![vec![0,0],vec![1,0],vec![2,0]]);
        assert_eq!(result, 2);
    }
}