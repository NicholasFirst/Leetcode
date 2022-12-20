
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn min_start_value(nums: Vec<i32>) -> i32 {
        let mut ans = 1;
        let mut total = 0;
        for (_, num) in nums.iter().enumerate() {
            total += num;
            if -total >= ans {
                ans = -total + 1;
            }
        }
        ans
    }
}

#[cfg(test)]
mod test{

    use super::Solution;

    #[test]
    fn test(){
        let nums = vec![-3,2,-3,4,2];
        let res = Solution::min_start_value(nums);
        assert_eq!(res, 5);
    }

    #[test]
    fn test1() {
        let nums = vec![1,2];
        let res = Solution::min_start_value(nums);
        assert_eq!(res, 1);
    }

    #[test]
    fn test2() {
        let nums = vec![1,-2,-3];
        let res = Solution::min_start_value(nums);
        assert_eq!(res, 5);
    }
}