
struct Solution;


impl Solution {
    // https://leetcode.cn/problems/special-positions-in-a-binary-matrix/
    #[allow(dead_code)]
    pub fn num_special(mat: Vec<Vec<i32>>) -> i32 {
        let rows = mat.len();
        let cols = mat[0].len();
        let mut row1 = vec![0; rows];
        let mut col1 = vec![0; cols];
        let mut ret = 0;

        for r in 0..rows {
            for c in 0..cols {
                if mat[r][c] == 1 {
                    row1[r] += 1;
                    col1[c] += 1;
                }
            }
        }
        for r in 0..rows {
            for c in 0..cols {
                if mat[r][c] == 1 && row1[r] + col1[c] == 2 {
                    ret += 1;
                }
            }
        }
        ret
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test() {
        let mat = vec![
            vec![1,0,0],
            vec![0,0,1],
            vec![1,0,0]
        ];
        let res = Solution::num_special(mat);
        assert_eq!(res, 1);

    }

    #[test]
    fn test1() {
        let mat = vec![
            vec![1,0,0],
            vec![0,1,0],
            vec![0,0,1]
        ];
        let res = Solution::num_special(mat);
        assert_eq!(res, 3);
    }
}