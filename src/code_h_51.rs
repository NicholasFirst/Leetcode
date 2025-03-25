use std::io;
use std::io::Write;

struct Solution;

impl Solution {
    // https://leetcode.cn/problems/n-queens/?envType=daily-question&envId=2025-03-25
    #[allow(dead_code)]
    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        let mut res: Vec<Vec<String>> = vec![];
        Solution::dfs(n, 0, vec![], &mut res);
        res
    }

    fn dfs(n: i32, row: i32, mut cols: Vec<i32>, res: &mut Vec<Vec<String>>) {
        if row == n {
            res.push(Solution::generate_board(n, &cols));
        }
        for col in 0..n {
            if !Solution::is_valid(&cols, row, col) {
                continue;
            }
            cols.push(col);
            Solution::dfs(n, row + 1, cols.clone(), res);
            cols.pop();
        }

    }

    fn is_valid(cols: &Vec<i32>, row: i32, col: i32) -> bool {
        for (i, &c) in cols.iter().enumerate() {
            if c == col || (row - i as i32).abs() == (col - c).abs() {
                return false;
            }
        }
        true
    }

    fn generate_board(n: i32, cols: &Vec<i32>) -> Vec<String> {
        let mut board: Vec<String> = vec![];
        for &col in cols {
            let mut row = ".".repeat(n as usize);
            row.replace_range(col as usize..col as usize + 1, "Q");
            board.push(row);
        }
        board
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        // assert_eq!(Solution::solve_n_queens(4), vec![vec![".Q..".to_string(), "...Q".to_string(), "Q...".to_string(), "..Q.".to_string()], vec!["..Q.".to_string(), "Q...".to_string(), "...Q".to_string(), ".Q..".to_string()]]);
        // assert_eq!(Solution::solve_n_queens(1), vec![vec!["Q".to_string()]]);
        let res = Solution::solve_n_queens(5);
        for item in res {
            println!("{:?}", item);
        }
    }
}