struct Solution;

impl Solution {

    // 1536. 排布二进制网格的最少交换次数
    #[allow(dead_code)]
    fn min_swaps(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        // 计算每一行末尾连续0的个数
        let mut trailing_zeros = Vec::with_capacity(n);
        for row in &grid {
            let mut count = 0;
            for &num in row.iter().rev() {
                if num == 0 {
                    count += 1;
                } else {
                    break;
                }
            }
            trailing_zeros.push(count);
        }

        let mut swaps = 0;
        for i in 0..n {
            // 第i行需要至少n-1-i个末尾0
            let required = n - 1 - i;
            // 找到第一个满足条件的行
            let mut j = i;
            while j < n && trailing_zeros[j] < required {
                j += 1;
            }
            // 如果找不到满足条件的行，返回-1
            if j == n {
                return -1;
            }
            // 计算需要交换的次数
            swaps += (j - i) as i32;
            // 将找到的行移动到当前位置
            let target = trailing_zeros.remove(j);
            trailing_zeros.insert(i, target);
        }
        swaps
    }
}


#[cfg(test)]
mod test{
    use super::Solution;

    #[test]
    fn test() {
        let grid = vec![
            vec![0, 0, 1],
            vec![1, 1, 0],
            vec![1, 0, 0]
        ];
        let ans = Solution::min_swaps(grid);
        assert_eq!(ans, 3);
    }

    #[test]
    fn test1() {
        let grid = vec![
            vec![0, 1, 1, 0],
            vec![0, 1, 1, 0],
            vec![0, 1, 1, 0],
            vec![0, 1, 1, 0]
        ];
        let ans = Solution::min_swaps(grid);
        assert_eq!(ans, -1);
    }

    #[test]
    fn test2() {
        let grid = vec![
            vec![1, 0, 0],
            vec![1, 1, 0],
            vec![1, 1, 1]
        ];
        let ans = Solution::min_swaps(grid);
        assert_eq!(ans, 0);
    }
}