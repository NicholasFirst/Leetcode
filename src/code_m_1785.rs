struct Solution;

impl Solution {
    // https://leetcode.cn/problems/minimum-elements-to-add-to-form-a-given-sum/
    #[allow(dead_code)]
    pub fn min_elements(nums: Vec<i32>, limit: i32, goal: i32) -> i32 {
        ((nums.iter().map(|&x| x as i64).sum::<i64>() - goal as i64).abs() as f64 / limit as f64)
            .ceil() as i32
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test() {
        let nums = vec![1,-1,1];
        let limit = 3;
        let goal = -4;
        let res = Solution::min_elements(nums, limit, goal);
        assert_eq!(2, res);
    }

    #[test]
    fn test1() {
        let nums = vec![1,-10,9,1];
        let limit = 100;
        let goal = 0;
        let res = Solution::min_elements(nums, limit, goal);
        assert_eq!(1, res);
    }


    #[test]
    fn test2() {
        let nums = vec![0,-10,-10,-10,10,7,-10,2];
        let limit = 10;
        let goal = 109765576;
        let res = Solution::min_elements(nums, limit, goal);
        assert_eq!(10976560, res);
    }
}
