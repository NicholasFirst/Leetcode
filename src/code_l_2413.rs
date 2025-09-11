struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn smallest_even_multiple(n: i32) -> i32 {
        // if n % 2 == 0 {
        //     n
        // } else {
        //     2 * n
        // }
        n << (n & 1)
    }
}

#[cfg(test)]
mod test{
    use super::Solution;

    #[test]
    fn test(){
        let res = Solution::smallest_even_multiple(6);
        assert_eq!(res, 6);

        let res = Solution::smallest_even_multiple(5);
        assert_eq!(res, 10);
    }   
}
