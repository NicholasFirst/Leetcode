struct Solution;

impl Solution {
    // https://leetcode.cn/problems/final-value-of-variable-after-performing-operations/
    #[allow(dead_code)]
    pub fn final_value_after_operations(operations: Vec<String>) -> i32 {
        operations.iter().fold(0, |ans, op| {
            if op.chars().nth(1).unwrap() == '+' {
                ans + 1
            } else {
                ans - 1
            }
        })
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test() {
        let operations = vec!["--X".to_string(), "X++".to_string(), "X++".to_string()];
        let res = Solution::final_value_after_operations(operations);
        assert_eq!(res, 1);
    }
}
