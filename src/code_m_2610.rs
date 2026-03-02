use std::collections::HashMap;

struct Solution;

impl Solution {

    #[allow(dead_code)]
    pub fn find_matrix(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res: Vec<Vec<i32>> = vec![];
        let mut map: HashMap<i32, usize> = HashMap::new();
        let mut max_count = 0;
        nums.iter().for_each(|&i| {
            let entry = map.entry(i).or_insert(0);
            *entry += 1;
            max_count = max_count.max(*entry);

        });
        for _ in 0..max_count {
            let mut row = vec![];
            for (k, v) in map.iter_mut() {
                if *v > 0 {
                    row.push(*k);
                    *v -= 1;
                }
            }
            res.push(row);
        }
        res
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2610() {
        let target= vec![1,3,4,1,2,3,1];
        let mut res = Solution::find_matrix(target);
        res.iter_mut().for_each(|iter| {
            iter.sort();
        });
        assert_eq!(res, vec![vec![1,2,3,4], vec![1,3], vec![1]]);
    }
}