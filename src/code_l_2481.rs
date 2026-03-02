struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn number_of_cuts(n: i32) -> i32 {
        // if n == 1 {
        //     0
        // }else if n % 2 == 0 {
        //     n / 2
        // } else {
        //     n
        // }
        if n == 1 { 0 } else if (n & 1) == 1 { n } else { n >> 1 }
    }
}


#[cfg(test)]
mod test{
    use super::Solution;

    #[test]
    fn test() {
        let n = 10;
        let m = 11;
        let res1 = Solution::number_of_cuts(n);
        let res2 = Solution::number_of_cuts(m);
        assert_eq!(res1, 5);
        assert_eq!(res2, 11);
    }
    
}