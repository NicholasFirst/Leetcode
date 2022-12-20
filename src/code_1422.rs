
struct Solution;

impl Solution{
    
    #[allow(dead_code)]
    pub fn max_score(s: String) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod test{
    use super::Solution;

    #[test]
    fn test() {
        let s = "011101".to_string();
        let res = Solution::max_score(s);
        assert_eq!(5, res);
    }
}