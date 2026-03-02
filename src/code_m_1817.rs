struct Solution;

use std::collections::{HashMap, HashSet};

impl Solution {

    // https://leetcode.cn/problems/finding-the-users-active-minutes/
    #[allow(dead_code)]
    pub fn finding_users_active_minutes(logs: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
        logs.into_iter().fold(HashMap::new(), |mut map, log| {
            map.entry(log[0]).or_insert(HashSet::new()).insert(log[1]);
            map
        })
        .into_values()
        .fold(vec![0; k as usize], |mut ans, f| {
            ans[f.len() - 1] += 1;
            ans
        })
    }
}

#[cfg(test)]
mod test{
    use super::Solution;

    #[test]
    fn test() {
        let logs = vec![vec![0,5],vec![1,2],vec![0,2],vec![0,5],vec![1,3]];
        let k = 5;
        let ans = Solution::finding_users_active_minutes(logs, k);
        assert_eq!(vec![0,2,0,0,0], ans);
    }
}