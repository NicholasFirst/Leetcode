struct Solution;

impl Solution {
    // https://leetcode.cn/problems/sudoku-solver/solutions/
    #[allow(dead_code)]
    pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
        let mut row = vec![vec![false; 9]; 9];
        let mut col = vec![vec![false; 9]; 9];
        let mut block = vec![vec![false; 9]; 9];
        let mut rest = vec![];
        for i in 0..9 {
            for j in 0..9 {
                match board[i][j] {
                    '.' => rest.push((i, j)),
                    _ => {
                        let n = (board[i][j] as u8 - b'1') as usize;
                        row[i][n] = true;
                        col[j][n] = true;
                        block[i / 3 * 3 + j / 3][n] = true;
                    }
                }
            }
        }
        dfs(board, &rest, &mut row, &mut col, &mut block);
    }
}    

fn dfs(
    board: &mut Vec<Vec<char>>,
    rest: &[(usize, usize)],
    row: &mut Vec<Vec<bool>>,
    col: &mut Vec<Vec<bool>>,
    block: &mut Vec<Vec<bool>>,
) -> bool {
    if let Some((i, j)) = rest.first() {
        let (i, j) = (*i, *j);
        for x in 0..9 {
            if !row[i][x] && !col[j][x] && !block[i / 3 * 3 + j / 3][x] {
                row[i][x] = true;
                col[j][x] = true;
                block[i / 3 * 3 + j / 3][x] = true;
                board[i][j] = (x as u8 + b'1') as char;
                if dfs(board, &rest[1..], row, col, block) {
                    return true;
                }
                row[i][x] = false;
                col[j][x] = false;
                block[i / 3 * 3 + j / 3][x] = false;
            }
        }
        false
    } else {
        true
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test() {
        let mut board = vec![
            vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ];
        Solution::solve_sudoku(&mut board);
        let result = vec![
            vec!['5', '3', '4', '6', '7', '8', '9', '1', '2'],
            vec!['6', '7', '2', '1', '9', '5', '3', '4', '8'],
            vec!['1', '9', '8', '3', '4', '2', '5', '6', '7'],
            vec!['8', '5', '9', '7', '6', '1', '4', '2', '3'],
            vec!['4', '2', '6', '8', '5', '3', '7', '9', '1'],
            vec!['7', '1', '3', '9', '2', '4', '8', '5', '6'],
            vec!['9', '6', '1', '5', '3', '7', '2', '8', '4'],
            vec!['2', '8', '7', '4', '1', '9', '6', '3', '5'],
            vec!['3', '4', '5', '2', '8', '6', '1', '7', '9'],
        ];
        assert_eq!(board, result);
    }
}
