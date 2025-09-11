struct Solution;

static MOD: i64 = 1_000_000_000 + 7;
use std::collections::VecDeque;

impl Solution {
    // https://leetcode.cn/problems/sum-of-subarray-minimums/solution/
    #[allow(unused)]
    #[allow(dead_code)]
    pub fn sum_subarray_mins(arr: Vec<i32>) -> i32 {
        let mut d: VecDeque<(usize, i32)> = VecDeque::new();
        let mut lr: Vec<(i64, i64)> = Vec::with_capacity(arr.len());
        arr.iter().enumerate().for_each(|(idx1, &x)| {
            while let Some((idx2, v)) = d.back() {
                match *v >= x {
                    true => {
                        lr[*idx2].1 = idx1 as i64;
                        d.pop_back();
                    }
                    false => break,
                }
            }
            match d.back() {
                Some((idx2, _)) => lr.push((*idx2 as i64, arr.len() as i64)),
                None => lr.push((-1, arr.len() as i64)),
            }
            d.push_back((idx1, x));
        });
        (lr.into_iter().enumerate().fold(0i64, |acc, (idx, (l, r))| {
            acc + (idx as i64 - l) * (r - idx as i64) * arr[idx] as i64
        }) % MOD) as i32
    }
}

#[cfg(test)]
mod test{
    use super::Solution;

    #[test]
    fn test() {
        let arr = vec![3,1,2,4];
        let ans = Solution::sum_subarray_mins(arr);
        assert_eq!(17, ans);
    }
} 