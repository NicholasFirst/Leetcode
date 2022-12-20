struct Solution;

impl Solution {
    /// https://leetcode.cn/problems/rotate-image/
    #[allow(dead_code)]
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        //1. 上下翻转
        matrix.reverse();
        // 左上右下对角线翻转
        for i in 0..matrix.len() {
            for j in i..matrix.len() {
                if i != j {
                    let tmp = matrix[i][j];
                    matrix[i][j] = matrix[j][i];
                    matrix[j][i] = tmp;
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let mut matrix = vec![
            vec![5, 1, 9, 11],
            vec![2, 4, 8, 10],
            vec![13, 3, 6, 7],
            vec![15, 14, 12, 16],
        ];
        Solution::rotate(&mut matrix);

        let res = vec![
            vec![15, 13, 2, 5],
            vec![14, 3, 4, 1],
            vec![12, 6, 8, 9],
            vec![16, 7, 10, 11],
        ];
        assert_eq!(matrix, res);
    }

    #[test]
    fn test1() {
        let mut matrix = vec![vec![1]];
        Solution::rotate(&mut matrix);
        assert_eq!(matrix, vec![vec![1]]);
    }

    use std::collections::HashSet;
    /// https://leetcode.cn/leetbook/read/array-and-string/ciekh/
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        let mut row = HashSet::new();
        let mut line = HashSet::new();
        matrix.iter().enumerate().for_each(|(idx, num)| {
            num.iter().enumerate().for_each(|(idx1, num1)| {
                if num1 == &0 {
                    row.insert(idx);
                    line.insert(idx1);
                }
            })
        });

        for i in 0..matrix.len() {
            for j in 0..matrix[i].len() {
                if row.contains(&i) || line.contains(&j) {
                    matrix[i][j] = 0;
                }
            }
        }
    }

    #[test]
    fn test3 () {
        let mut matrix = vec![
            vec![0,1,2,0],
            vec![3,4,5,2],
            vec![1,3,1,5]
          ];
        set_zeroes(&mut matrix);
        let res = vec![
            vec![0,0,0,0],
            vec![0,4,5,0],
            vec![0,3,1,0]
          ];
        assert_eq!(matrix, res);
    }
}
