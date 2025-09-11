struct Solution;

use crate::node::tree_node::TreeNode;
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    // https://leetcode.cn/problems/longest-univalue-path/
    // 不是我自己做的....
    #[allow(dead_code)]
    pub fn longest_univalue_path(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut ans = 0;
        Self::dfs(root.as_ref(), &mut ans, -1);
        ans
    }

    fn dfs(root: Option<&Rc<RefCell<TreeNode>>>, max: &mut i32, parent_val: i32) -> i32 {
        if let Some(r) = root {
            let left = Self::dfs(r.borrow().left.as_ref(), max, r.borrow().val);
            let right = Self::dfs(r.borrow().right.as_ref(), max, r.borrow().val);

            *max = (*max).max(left + right);

            if parent_val == r.borrow().val {
                left.max(right) + 1
            } else {
                0
            }
        } else {
            0
        }
    }
}

#[cfg(test)]
mod test{

    use super::Solution;
    use crate::node::tree_node::TreeNode;

    #[test]
    fn test() {
        let vec = vec![Some(5),Some(4),Some(5),Some(1),Some(1),None,Some(5)];
        let root = TreeNode::from(vec).unwrap();
        let res = Solution::longest_univalue_path(root);
        assert_eq!(res, 2);

    }
}