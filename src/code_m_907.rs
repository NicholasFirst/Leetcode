struct Solution;


impl Solution {
    // https://leetcode.cn/problems/sum-of-subarray-minimums/solution/
    #[allow(dead_code)]
    pub fn sum_subarray_mins(arr: Vec<i32>) -> i32 {
        let n = arr.len();
        let mut stack: Vec<usize> = Vec::new();
        let left = vec![0; n];
        let right = vec![0; n];
        for i in 0..n {
            while !stack.is_empty() && arr[i] <= arr[*stack.first().unwrap()] {
                stack.pop();
            }
            stack.push(i);
        }
        stack.clear();
        todo!()
    }
}

#[cfg(test)]
mod test{
    use super::Solution;

    #[test]
    fn test() {
        let arr = vec![3,1,2,4];
        let ans = Solution::sum_subarray_mins(arr);
        assert_eq!(17, ans);
    }
} 