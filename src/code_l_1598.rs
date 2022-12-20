struct Solution;

impl Solution {
    // https://leetcode.cn/problems/crawler-log-folder/
    #[allow(dead_code)]
    pub fn min_operations(logs: Vec<String>) -> i32 {
        let mut ans = 0;
        for iter in logs {
            if iter == "./" {
                
            }else if iter == "../" {
                if ans > 0 {
                    ans -= 1;
                }
            }else{
                ans += 1;
            }
        }
        ans
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test() {
        let logs = vec![
            "d1/".to_string(),
            "d2/".to_string(),
            "../".to_string(),
            "d21/".to_string(),
            "./".to_string(),
        ];
        let res = Solution::min_operations(logs);
        assert_eq!(res, 2);
    }
}
