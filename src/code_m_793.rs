struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn preimage_size_fzf(k: i32) -> i32 {
        let (mut l, mut r) = (0, i32::MAX);
        while l < r {
            let m = l + ((r - l) >> 1);
            let (mut i, mut c) = (5, m);
            while m / i > 0 {
                c += m / i;
                i *= 5;
            }
            if c == k { return 5; }
            if c < k { l = m + 1; } else { r = m; }
        }
        0
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test() {
        let ans = Solution::preimage_size_fzf(5);
        assert_eq!(ans, 0);
    }
}