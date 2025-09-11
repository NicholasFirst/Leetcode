struct Solution;

impl Solution {
    //https://leetcode.cn/problems/find-missing-and-repeated-values/?envType=daily-question&envId=2024-05-31
    #[allow(dead_code)]
    pub fn find_missing_and_repeated_values(grid: Vec<Vec<i32>>) -> Vec<i32> {
        use std::collections::HashSet;
        let n = grid.len() as i32;
        let mut set = HashSet::new();
        let mut res = vec![0; 2];
        for x in grid.iter().flatten() {
            if set.contains(x) {
                res[0] = *x;
            }
            set.insert(*x);
        }
        res[1] = (1..=n * n).find(|x| !set.contains(x)).unwrap();
        res
    }
}

//test
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test1() {
        let grid = vec![vec![1, 3], vec![2, 2]];
        let res = Solution::find_missing_and_repeated_values(grid);
        assert_eq!(res, vec![2, 4]);
    }

    #[test]
    fn test2() {
        let grid = vec![vec![9, 1, 7], vec![8, 9, 2], vec![3, 4, 6]];
        let res = Solution::find_missing_and_repeated_values(grid);
        assert_eq!(res, vec![9, 5]);
    }
}
