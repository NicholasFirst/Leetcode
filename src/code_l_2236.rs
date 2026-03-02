use crate::node::tree_node::TreeNode;

struct Solution;

use std::rc::Rc;
use std::cell::RefCell;
impl Solution {

    // https://leetcode.cn/problems/root-equals-sum-of-children/description/
    #[allow(dead_code)]
    pub fn check_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        match root {
            Some(node) => match (node.borrow().left.as_ref(), node.borrow().right.as_ref()) {
                (Some(left), Some(right)) => {
                    left.borrow().val + right.borrow().val == node.borrow().val
                }
                _ => true,
            },
            None => true,
        }
    }
}

#[cfg(test)]
mod test{
    use crate::node::tree_node::TreeNode;
    use super::Solution;

    #[test]
    fn test() {
        let vec = vec![Some(10), Some(4), Some(6)];
        let root = TreeNode::from(vec).unwrap();
        assert!(Solution::check_tree(root));
    }
}