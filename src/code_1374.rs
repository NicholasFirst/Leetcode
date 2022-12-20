struct Solution;

impl Solution{
    /// https://leetcode.cn/problems/generate-a-string-with-characters-that-have-odd-counts/
    #[allow(dead_code)]
    pub fn generate_the_string(n: i32) -> String {
        let mut res = "".to_string();
        if n <= 0 {
            return res;
        }
        if n % 2 == 0 {
            for _ in 0..(n-1) {
                res.push('a');
            }
            res.push('b');
            return res;
        }else{
            for _ in 0..n {
                res.push('a');
            }
            return res;
        }
    }
}

#[cfg(test)]
mod test{
    use super::Solution;

    #[test]
    fn test() {
        let res = Solution::generate_the_string(5);
        assert_eq!("aaaaa".to_string(), res);
        let res = Solution::generate_the_string(6);
        assert_eq!("aaaaab".to_string(), res);
    }
}