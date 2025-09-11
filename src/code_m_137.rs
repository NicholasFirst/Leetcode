struct Solution;

impl Solution{

    // https://leetcode.cn/problems/single-number-ii/description/
    #[allow(dead_code)]
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut a = 0;
        let mut b = 0;
        for i in nums {
            dbg!(a, b);
            println!();
            b = !a & (b ^ i);
            a = !b & (a ^ i);
        }
        b
    }
}

#[cfg(test)]
mod test{
    use super::Solution;

    #[test]
    fn test() {
        let nums = vec![0,1,0,1,0,1,99];
        let ans = Solution::single_number(nums);
        assert_eq!(99, ans);
    }

    #[test]
    fn test1() {
        let x = 99;
        let b = !x;
        println!("{b}");
    }

    #[test]
    fn test2() {
        let a = 1;
        let b = 2;
        let c = 7;
        let d = a ^ b;
        dbg!(d);
        println!("{:b}", c);
    }
}