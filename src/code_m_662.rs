use crate::node::tree_node::TreeNode;
struct Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {

    /// 这题不算!!!
    #[allow(dead_code)]
    pub fn width_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() {
            return 0
        }
        let mut ans = 1;
        let mut que = vec![(0, root)];
        while !que.is_empty() {
            ans = ans.max(que.last().unwrap().0 - que[0].0 + 1);
            let mut tmp = vec![];
            for (i, r) in que {
                let r = r.as_ref().unwrap().borrow();
                if r.left.is_some() {
                    tmp.push((i * 2, r.left.clone()));
                }
                if r.right.is_some() {
                    tmp.push((i * 2 + 1, r.right.clone()));
                }
            }
            que = tmp;
        }
        ans
    }
}

#[cfg(test)]
mod test {
    use super::Solution;
    use crate::node::tree_node::TreeNode;

    #[test]
    fn test() {
        let root = vec![
            Some(1),
            Some(3), Some(2),
            Some(5), None, None, Some(9),
            Some(6), None, None, None, None, None, Some(7), None,
        ];
        let root = TreeNode::from(root).unwrap();
        let res = Solution::width_of_binary_tree(root);
        assert_eq!(res, 7);
    }
}
