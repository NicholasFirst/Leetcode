struct Solution;

impl Solution {

    // https://leetcode.cn/problems/nth-magical-number/solutions/?languageTags=rust
    #[allow(dead_code)]
    pub fn nth_magical_number(n: i32, a: i32, b: i32) -> i32 {
        const MOD: usize = 1e9 as usize + 7;
        let lcm = (a * b / Solution::gcd(a, b)) as usize;
        let (a, b, n) = (a as usize, b as usize, n as usize);
        let check = |target: usize| -> bool {
            (target / a) + (target / b) - (target / (lcm)) >= n
        };
        let (mut l, mut r) = (0, usize::MAX);
        while l < r {
            let mid = l + ((r - l) >> 2);
            if check(mid) {
                r = mid;
            } else {
                l = mid + 1;
            }
        }
        (l % MOD) as i32
    }

    /// 计算最大公约数
    fn gcd(a: i32, b: i32) -> i32 {
        if b == 0 {
            a
        }else {
            Self::gcd(b, a % b)
        }
    }
}

#[cfg(test)]
mod test{
    use super::Solution;

    #[test]
    fn test() {
        let n = 4;
        let a = 2;
        let b = 3;
        let res = Solution::nth_magical_number(n, a, b);
        assert_eq!(6, res);
    }

    #[test]
    fn test_gcd() {
        let ans = Solution::gcd(75, 200);
        assert_eq!(ans, 25);
    }
}