struct Solution;

impl Solution {
 
    // https://leetcode.cn/problems/greatest-english-letter-in-upper-and-lower-case/
    #[allow(dead_code)]
    pub fn greatest_letter(s: String) -> String {
        for ch in ('A'..='Z').rev() {
            if s.contains(ch) && s.contains(ch.to_ascii_lowercase()) {
                return ch.to_string();
            }
        }
        String::new()
    }
}

#[cfg(test)]
mod test{
    use super::Solution;

    #[test]
    fn test(){
        let s = "arRAzFif".to_string();
        let res = Solution::greatest_letter(s);
        assert_eq!("R".to_string(), res);
    }
    
    #[test]
    fn test1(){
        let s = "AbCdEfGhIjK".to_string();
        let res = Solution::greatest_letter(s);
        assert_eq!("".to_string(), res);
    }
}