use crate::node::tree_node::TreeNode;

struct Solution;

use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    /// https://leetcode.cn/problems/maximum-binary-tree/
    #[allow(dead_code)]
    pub fn construct_maximum_binary_tree(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        TreeNode::construct_maximum_binary_tree(nums)
    }
}

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn test() {
        let nums = vec![3,2,1,6,0,5];
        let res = Solution::construct_maximum_binary_tree(nums);
        let value = vec![
            Some(6),
            Some(3), Some(5),
            None, Some(2), Some(0), None,
            None, None, None, Some(1), None, None, None, None
        ];
        let real = TreeNode::from(value).unwrap();
        assert_eq!(res, real);
    }
}