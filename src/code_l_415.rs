struct Solution;

impl Solution {

    // https://leetcode.cn/problems/add-strings/
    #[allow(dead_code)]
    pub fn add_strings(num1: String, num2: String) -> String {
        let len = num1.len().max(num2.len());
        let mut num1 = num1.chars().rev();
        let mut num2 = num2.chars().rev();
        let mut carry = 0;
        let mut res = std::collections::VecDeque::new();
        for _ in 0..len {
            let f = num1.next().map_or(0, |v| v.to_digit(10).unwrap());
            let s = num2.next().map_or(0, |v| v.to_digit(10).unwrap());
            res.push_front((f + s + carry) % 10);
            if f + s + carry >= 10 {
                carry = 1;
            }else {
                carry = 0;
            }
        }
        if carry != 0 {
            res.push_front(1);
        }
        res.iter().map(|s| s.to_string()).collect()
    }
}

#[cfg(test)]
mod test{
    use super::Solution;

    #[test]
    fn test() {
        let num1 = "456".to_string();
        let num2 = "77".to_string();
        let res = Solution::add_strings(num1, num2);
        assert_eq!(res, "533".to_string());
    }

    #[test]
    fn test1() {
        let num1 = "956".to_string();
        let num2 = "77".to_string();
        let res = Solution::add_strings(num1, num2);
        assert_eq!(res, "1033".to_string());
    }
}