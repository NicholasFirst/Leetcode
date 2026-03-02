struct Solution;

impl Solution {
    // https://leetcode.cn/problems/score-of-parentheses/
    #[allow(dead_code)]
    pub fn score_of_parentheses(s: String) -> i32 {
        let mut stack = vec![0];
        for c in s.chars() {
            if c == '(' {
                stack.push(0);
            }else{
                let top = stack.pop().unwrap();
                let last = stack.last_mut().unwrap();
                *last += (top * 2).max(1);
            }
        }
        *stack.last().unwrap()
    }
}


#[cfg(test)]
mod test{

    use super::Solution;

    #[test]
    fn test() {
        let s = "(()(()))".to_string();
        let ans = Solution::score_of_parentheses(s);
        assert_eq!(ans, 6);
    }

    #[test]
    fn test1() {
        let s = "(())".to_string();
        let ans = Solution::score_of_parentheses(s);
        assert_eq!(ans, 2);
    }
}