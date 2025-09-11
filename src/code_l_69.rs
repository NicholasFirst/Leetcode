struct Solution;

impl Solution {
    //https://leetcode.cn/problems/sqrtx/
    #[allow(dead_code)]
    pub fn my_sqrt(x: i32) -> i32 {
        let x_half = 0.5 * x as f64;
        let mut i = (x as f64).to_bits();
        i = 0x1FF7A3BEA91D9B1B + (i >> 1);
        let mut f = f64::from_bits(i);
        f = 0.5 * f + x_half / f;
        f = 0.5 * f + x_half / f;
        f = 0.5 * f + x_half / f;
        f as i32
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test() {
        let x = 625;
        let ans = Solution::my_sqrt(x);
        assert_eq!(25, ans);
    }
}
