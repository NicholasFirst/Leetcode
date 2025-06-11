struct Solution;

impl Solution {
    // https://leetcode.cn/problems/valid-perfect-square/
    #[allow(dead_code)]
    pub fn is_perfect_square(num: i32) -> bool {
        let x_half = 0.5 * num as f32;
        let mut i = (num as f32).to_bits();
        i = 0x1FBD1DF5 + (i >> 1);
        let mut f = f32::from_bits(i);
        f = 0.5 * f + x_half / f; // 牛顿迭代步 手动迭代两次
        f = 0.5 * f + x_half / f;
        (f as i32).pow(2) == num
        // ????
    }
}

#[cfg(test)]
mod tests {
    
    use super::*;

    #[test]
    fn test_is_perfect_square() {
        assert_eq!(Solution::is_perfect_square(16), true);
        assert_eq!(Solution::is_perfect_square(14), false);
    }
}