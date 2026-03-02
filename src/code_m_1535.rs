struct Solution;

impl Solution {
    #[allow(dead_code)]
    // https://leetcode.cn/problems/find-the-winner-of-an-array-game/description/?envType=daily-question&envId=2024-05-19
    pub fn get_winner(arr: Vec<i32>, k: i32) -> i32 {
        if arr.len() <= k as usize {
            return *arr.iter().max().unwrap();
        }
        let mut max_val = arr[0];
        let mut max_idx = 0;
        for i in 1..arr.len() {
            if arr[i] > max_val {
                max_val = arr[i];
                max_idx = 0;
            }
            max_idx += 1;
            if max_idx == k as usize {
                return max_val;
            }
        }
        max_val
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1535() {
        let arr = vec![2,1,3,5,4,6,7];
        let k = 2;
        let res = Solution::get_winner(arr, k);
        assert_eq!(res, 5);
    }

    #[test]
    fn test_1535_2() {
        let arr = vec![3,2,1];
        let k = 10;
        let res = Solution::get_winner(arr, k);
        assert_eq!(res, 3);
    }

    #[test]
    fn test_1535_3() {
        let arr = vec![1,9,8,2,3,7,6,4,5];
        let k = 7;
        let res = Solution::get_winner(arr, k);
        assert_eq!(res, 9);
    }

    #[test]
    fn test_1535_4() {
        let arr = vec![1,11,22,33,44,55,66,77,88,99];
        let k = 1000000000;
        let res = Solution::get_winner(arr, k);
        assert_eq!(res, 99);
    }

    #[test]
    fn test_1535_5() {
        let arr = vec![1,25,35,42,68,70];
        let k = 2;
        let res = Solution::get_winner(arr, k);
        assert_eq!(res, 70);
    }
}