struct Solution;

impl Solution {
    // https://leetcode.cn/problems/largest-rectangle-in-histogram/
    #[allow(dead_code)]
    pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        let mut ans = 0;
        let mut stack = vec![0];
        let heights = [0].iter().chain(&heights).chain(&[0]).collect::<Vec<_>>();
        for (i, &h) in heights.iter().enumerate().skip(1) {
            while heights[*stack.last().unwrap()] > h {
                ans = ans.max(heights[stack.pop().unwrap()] * (i - stack.last().unwrap() - 1) as i32);
            }
            stack.push(i);
        }
        ans
    }
}

#[cfg(test)]
mod test{
    use super::Solution;

    #[test]
    fn test() {
        let heights = vec![2,1,5,6,2,3];
        let seq = Solution::largest_rectangle_area(heights);
        assert_eq!(seq, 10);
    }

    #[test]
    fn test1() {
        let heights = vec![2,4];
        let seq = Solution::largest_rectangle_area(heights);
        assert_eq!(seq, 4);
    }


    //Monotonic stack 单调栈
    fn monotonic_stack(nums: Vec<i32>) -> Vec<i32> {
        let mut stack = Vec::new();
        for i in nums {
            while !stack.is_empty() && stack.last().unwrap() < &i {
                stack.pop();
            }
            stack.push(i);
        }
        stack
    }

    #[test]
    fn test_monotonic_stack() {
        let nums = vec![3,4,2,6,4,5,2,3];
        let ans = monotonic_stack(nums);
        assert_eq!(ans, vec![6, 5, 3]);
    }
}