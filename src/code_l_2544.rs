struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn alternate_digit_sum(mut n: i32) -> i32 {
        let (mut ret, mut k) = (0, 1);
        while n != 0 {
            ret += k * (n % 10);
            k = -k;
            n /= 10;
        }
        -k * ret
    }
}

