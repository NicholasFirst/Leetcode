struct Solution;

impl Solution {
    // https://leetcode.cn/problems/count-asterisks/
    #[allow(dead_code)]
    pub fn count_asterisks(s: String) -> i32 {
        let mut ans = 0;
        let mut may = true;
        s.chars().for_each(|c| {
            if may && c == '*' {
                ans += 1;
            }
            if c == '|' {
                may = !may;
            }
        });
        ans
    }
}

#[cfg(test)]
mod test{
    use super::Solution;

    #[test]
    fn test() {
        let s = "l|*e*et|c**o|*de|".to_string();
        let ans = Solution::count_asterisks(s);
        assert_eq!(2, ans);
    }


    #[test]
    fn test1() {
        let s = "iamprogrammer".to_string();
        let ans = Solution::count_asterisks(s);
        assert_eq!(0, ans);
    }


    #[test]
    fn test2() {
        let s = "yo|uar|e**|b|e***au|tifu|l".to_string();
        let ans = Solution::count_asterisks(s);
        assert_eq!(5, ans);
    }
}