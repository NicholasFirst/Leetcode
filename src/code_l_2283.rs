struct Solution;

impl Solution {
    // https://leetcode.cn/problems/check-if-number-has-equal-digit-count-and-digit-value/
    #[allow(dead_code)]
    pub fn digit_count(num: String) -> bool {
        let mut count_map = [0u8; 10];
        num.bytes()
            .for_each(|c| count_map[(c - b'0') as usize] += 1);
        num.bytes()
            .enumerate()
            .all(|(i, c)| c - b'0' == count_map[i])
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test() {
        let num = "1210".to_string();
        let res = Solution::digit_count(num);
        assert!(res);
    }

    #[test]
    fn test1() {
        let num = "195365".to_string();
        let x = num.bytes();
        println!("{:?}", x);    //[49, 57, 53, 51, 54, 53]
    }

    #[test]
    fn test2() {
        let num = b"1992";
        
    }
}
