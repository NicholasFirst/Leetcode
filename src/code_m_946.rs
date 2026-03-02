struct Solution;

impl Solution {
    #[allow(dead_code)]
    // https://leetcode.cn/problems/validate-stack-sequences/
    pub fn validate_stack_sequences(pushed: Vec<i32>, popped: Vec<i32>) -> bool {
        let mut stack = Vec::with_capacity(pushed.len());
        let mut i = 0;

        for val in pushed {
            stack.push(val);
            while i < popped.len() && *stack.last().unwrap_or(&-1) == popped[i] {
                stack.pop();
                i += 1;
            }
        }
        stack.is_empty()
    }
}


#[cfg(test)]
mod test{
    
    use super::Solution;

    #[test]
    fn test() {
        let pushed = vec![1,2,3,4,5]; 
        let popped = vec![4,5,3,2,1];
        let res = Solution::validate_stack_sequences(pushed, popped);
        assert!(res);
    }


    #[test]
    fn test1() {
        let pushed = vec![1,2,3,4,5]; 
        let popped = vec![4,3,5,1,2];
        let res = Solution::validate_stack_sequences(pushed, popped);
        assert!(!res);
    }
}