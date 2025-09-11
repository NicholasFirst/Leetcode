
struct Solution;
use std::collections::HashMap;

impl Solution {
    /// https://leetcode.cn/problems/group-the-people-given-the-group-size-they-belong-to/
    #[allow(dead_code)]
    pub fn group_the_people(group_sizes: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res = vec![];
        let mut map = HashMap::new();
        for (i, num) in group_sizes.iter().enumerate() {
            let e = map.entry(num).or_insert(vec![]);
            e.push(i as i32);
            if e.len() == *num as usize {
                res.push(e.clone());
                map.remove(num);
            }
        }
        res
    }
}

#[cfg(test)]
mod test{

    use super::*;

    #[test]
    fn test() {
        let group_sizes = vec![3,3,3,3,3,1,3];
        let res = Solution::group_the_people(group_sizes);
        println!("{:?}", res);
    }
}