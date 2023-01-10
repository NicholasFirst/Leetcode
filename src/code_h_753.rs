use std::collections::{HashMap, HashSet};
struct Solution;

impl Solution {
    // https://leetcode.cn/problems/sudoku-solver/solutions/
    #[allow(dead_code)]
    pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
        let mut res_map = Self::fix_map(&board);
        Self::abrasor(&mut res_map, board);
        todo!()
    }

    fn fix_map(board: &Vec<Vec<char>>) -> HashMap<(usize, usize), HashSet<char>> {
        let chars: HashSet<char> = vec!['1', '2', '3', '4', '5', '6', '7', '8', '9'].drain(..).collect();
        let mut map = HashMap::new();
        board.iter().enumerate().for_each(|(i, vec)| {
            vec.iter().enumerate().for_each(|(j, c)| match c {
                '.' => {
                    map.insert((i, j), chars.clone());
                }
                _ => {}
            })
        });
        map
    }

    fn abrasor(map: &mut HashMap<(usize, usize), HashSet<char>>, board: &mut Vec<Vec<char>>) {
        board.iter().enumerate().for_each(|(i, vec)| {
            vec.iter().enumerate().for_each(|(j, c)| {
                map.iter_mut().for_each(|((k, v), val)| {
                    if k == &i || v == &j {
                        val.remove(c);
                    }
                })
            })
        });
        dbg!(map);
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
