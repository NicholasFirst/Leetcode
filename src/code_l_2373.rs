struct Solution;

impl Solution {
    // https://leetcode.cn/problems/largest-local-values-in-a-matrix/
    #[allow(dead_code)]
    pub fn largest_local(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let n = grid.len();
        let mut res: Vec<Vec<i32>> = vec![vec![0; n - 2]; n - 2];
        let direct = vec![
            vec![-1, -1],vec![-1, 0],vec![-1, 1],
            vec![0, -1],vec![0, 0],vec![0, 1],
            vec![1, -1],vec![1, 0],vec![1, 1],
        ];
        for row in 1..(n - 1) {
            for col in 1..(n - 1) {
                res[row - 1][col - 1] = direct.iter().map(
                    |offset| grid[(row as i32 + offset[0]) as usize][(col as i32 + offset[1]) as usize]
                ).max().unwrap()
            }
        }
        res
    }
}

#[cfg(test)]
mod test{
    use super::Solution;

    #[test]
    fn test() {
        let grid = vec![vec![9,9,8,1],vec![5,6,2,6],vec![8,2,6,4],vec![6,2,2,2]];
        let ans = Solution::largest_local(grid);
        assert_eq!(vec![vec![9,9],vec![8,6]], ans);
    }


    #[test]
    fn test1() {
        let grid = vec![vec![1,1,1,1,1],vec![1,1,1,1,1],vec![1,1,2,1,1],vec![1,1,1,1,1],vec![1,1,1,1,1]];
        let ans = Solution::largest_local(grid);
        assert_eq!(vec![vec![2,2,2],vec![2,2,2],vec![2,2,2]], ans);
    }
}