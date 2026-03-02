struct Solution;

impl Solution {
    // https://leetcode.cn/problems/pascals-triangle/description/
    #[allow(dead_code)]
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let mut res = vec![];
        for i in 0..num_rows as usize {
            res.push(vec![1; i + 1]);
            for j in 1..i {
                res[i][j] = res[i - 1][j - 1] + res[i - 1][j];
            }
        }
        res
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test() {
        let res = Solution::generate(5);
        assert_eq!(
            vec![
                vec![1],
                vec![1, 1],
                vec![1, 2, 1],
                vec![1, 3, 3, 1],
                vec![1, 4, 6, 4, 1]
            ],
            res
        );
    }
}
