struct Solution;

impl Solution {

    // https://leetcode.cn/problems/couples-holding-hands/
    #[allow(dead_code)]
    pub fn min_swaps_couples(mut row: Vec<i32>) -> i32 {
        let mut ans = 0;
        for i in (0..row.len()).step_by(2) {
            if row[i] % 2 == 0 {
                if row[i + 1] != row[i] + 1 {
                    ans += 1;
                    for j in i + 2..row.len() {
                        if row[j] == row[i] + 1 {
                            row.swap(i + 1, j);
                            break;
                        }
                    }
                }
            }
            else {
                if row[i + 1] != row[i] - 1 {
                    ans += 1;
                    for j in i + 2..row.len() {
                        if row[j] == row[i] - 1 {
                            row.swap(i + 1, j);
                            break;
                        }
                    }
                }
            }

        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_765() {
        assert_eq!(Solution::min_swaps_couples(vec![0, 2, 1, 3]), 1);
        assert_eq!(Solution::min_swaps_couples(vec![3, 2, 0, 1]), 0);
    }
}