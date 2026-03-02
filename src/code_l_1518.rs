struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn num_water_bottles(num_bottles: i32, num_exchange: i32) -> i32 {
        let mut ans = num_bottles;
        let mut empty = num_bottles;

        while empty >= num_exchange {
            let new_bottles = empty / num_exchange;
            ans += new_bottles;
            empty = empty % num_exchange + new_bottles;
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1518() {
        assert_eq!(Solution::num_water_bottles(9, 3), 13);
        assert_eq!(Solution::num_water_bottles(15, 4), 19);
    }
}