struct Solution;

impl Solution {

    #[allow(dead_code)]
    pub fn min_operations(s: String) -> i32 {
        let col:Vec<char> = (0..s.len()).map(|i| {
            if i % 2 == 0 {
                return '0';
            }else {
                return '1';
            }
        }).collect();
        let mut no = 0;
        let mut re = 0;
        s.chars().enumerate().for_each(|(i, c)| {
            if c != col[i] {
                re += 1;
            }else {
                no += 1;
            }
        });
        no.min(re)
    }
}


#[cfg(test)]
mod test{

    use super::Solution;

    #[test]
    fn test() {
        let s = "0100".to_string();
        let res = Solution::min_operations(s);
        assert_eq!(res, 1);
    }

    #[test]
    fn test1() {
        let s = "1111".to_string();
        let res = Solution::min_operations(s);
        assert_eq!(res, 2);
    }
} 