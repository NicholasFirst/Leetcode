struct Solution;

impl Solution {
    // https://leetcode.cn/problems/second-largest-digit-in-a-string/
    #[allow(dead_code)]
    pub fn second_highest(s: String) -> i32 {
        let mut max = -1;
        let mut sec = -1;
        for n in s.chars(){
            if n>='0' && n<='9'{
                let c = n as i32 - '0' as i32;
                if c>max{
                    sec = max;
                    max = c;
                }else if c!=max && c>sec{
                    sec = c;
                }
            }
        }
        return sec;
    }
}

#[cfg(test)]
mod test{
    use super::Solution;

    #[test]
    fn test() {
        let s = "dfa12321afd".to_string();
        let ans = Solution::second_highest(s);
        assert_eq!(ans, 2);
    }

    #[test]
    fn test1() {
        let s = "abc1111".to_string();
        let ans = Solution::second_highest(s);
        assert_eq!(ans, -1);
    }

    #[test]
    fn test2() {
        let s = "ck077".to_string();
        let ans = Solution::second_highest(s);
        assert_eq!(ans, 0);
    }
}