
struct Solution;

impl Solution {

    #[allow(dead_code)]
    pub fn reformat(s: String) -> String {
        let nums:Vec<char> = s.chars().filter(|c| c.is_numeric()).collect();
        let chars: Vec<char> = s.chars().filter(|c| !c.is_numeric()).collect();
        let num_len = nums.len() as i32;
        let char_len = chars.len() as i32;
        let mut first = num_len > char_len;
        let mut nums_iter = nums.iter();
        let mut chars_iter = chars.iter();
        if (num_len - char_len).abs() > 1 {
            return "".to_string();
        }
        let mut res = "".to_string();
        for _ in 0..s.len() {
            if first {
                res.push(nums_iter.next().unwrap().clone());
                first = !first;
            }else {
                res.push(chars_iter.next().unwrap().clone());
                first = !first;
            }
        }
        res
    }
}

#[cfg(test)]
mod test{

    use super::Solution;
    
    #[test]
    fn test() {
        let s = "a0b1c2".to_string();
        let res = Solution::reformat(s);
        assert_eq!("a0b1c2".to_string(), res);
    }

    #[test]
    fn test2() {
        let s = "leetcode".to_string();
        let res = Solution::reformat(s);
        assert_eq!("".to_string(), res);
    }

    #[test]
    fn test1() {
        let s = "a0b1c2".to_string();
        let bytes = s.as_bytes();
        println!("{:?}", bytes);
    }
} 