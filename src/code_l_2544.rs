struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn alternate_digit_sum(mut n: i32) -> i32 {
        let mut res = 0;
        let mut neg = 1;
        for i in n.to_string().split("") {
            if !i.is_empty() {
                res += i.parse::<i32>().unwrap() * neg;
                neg *= -1;
            }
        }
        res
        // let (mut ret, mut k) = (0, 1);
        // while n != 0 {
        //     ret += k * (n % 10);
        //     k = -k;
        //     n /= 10;
        // }
        // -k * ret
    }
}

#[cfg(test)]
mod test{
    use super::Solution;

    #[test]
    fn test(){
        let n = 111;
        let res = Solution::alternate_digit_sum(n);
        assert_eq!(1, res);
    }

}