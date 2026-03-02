struct Solution;

use std::collections::HashMap;

impl Solution {

    // https://leetcode.cn/problems/roman-to-integer/description/?languageTags=rust
    #[allow(dead_code)]
    fn roman_to_int(s: String) -> i32{
        let map = ['I', 'V', 'X', 'L', 'C', 'D', 'M']
            .iter()
            .zip([1, 5, 10, 50, 100, 500, 1000])
            .collect::<HashMap<_, _>>();
        s.chars().rev().fold(0, |acc, c| {
            acc + if acc > 4 * map[&c] { -map[&c] } else { map[&c] }
        })
    }
}


#[cfg(test)]
mod test{
    use super::Solution;

    #[test]
    fn test() {
        let s = "III".to_string();
        let ans = Solution::roman_to_int(s);
        assert_eq!(ans, 3);
    }

    #[test]
    fn test1() {
        let s = "IX".to_string();
        let ans = Solution::roman_to_int(s);
        assert_eq!(ans, 9);
    }
}
