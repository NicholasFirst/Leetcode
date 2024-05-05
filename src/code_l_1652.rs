struct Solution;

impl Solution {

    #[allow(dead_code)]
    // https://leetcode.cn/problems/defuse-the-bomb/description/?envType=daily-question&envId=2024-05-05
    pub fn decrypt(code: Vec<i32>, k: i32) -> Vec<i32> {
        let n = code.len();
        let mut ans = vec![0; n];
        let mut r = if k > 0 { k as usize + 1 } else { n };
        let k = k.abs() as usize;
        let mut s = code[r - k..r].iter().sum::<i32>(); // ans[0]
        for i in 0..n {
            ans[i] = s;
            s += code[r % n] - code[(r - k) % n];
            r += 1;
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let code = vec![5, 7, 1, 4];
        let k = 3;
        let ans = vec![12, 10, 16, 13];
        let t = Solution::decrypt(code, k);
        assert_eq!(t, ans);
    }
}