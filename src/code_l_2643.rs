struct Solution;

impl Solution {
    // https://leetcode.cn/problems/row-with-maximum-ones/?envType=daily-question&envId=2025-03-22
    #[allow(dead_code)]
    pub fn row_and_maximum_ones(mat: Vec<Vec<i32>>) -> Vec<i32> {
        let mut res = vec![0 as i32, 0];
        for (i, row) in mat.iter().enumerate() {
            let count = row.iter().filter(|x| **x == 1).count() as i32;
            if count > res[1] {
                res[0] = i as i32;
                res[1] = count;
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_row_and_maximum_ones() {
        assert_eq!(Solution::row_and_maximum_ones(vec![vec![0, 1], vec![1, 0]]), vec![0, 1]);
        assert_eq!(Solution::row_and_maximum_ones(vec![vec![0, 0, 0], vec![0, 1, 1]]), vec![1, 2]);
        assert_eq!(Solution::row_and_maximum_ones(vec![vec![0, 0], vec![1, 1], vec![0, 0]]), vec![1, 2]);
    }
}