use std::rc::Rc;
use std::cell::RefCell;

macro_rules! tree {
    ($val:expr, $left:expr, $right:expr) => {
        Some(Rc::new(RefCell::new(TreeNode{val: $val, left: $left, right: $right})))
    };
}

//Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
  pub val: i32,
  pub left: Option<Rc<RefCell<TreeNode>>>,
  pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[allow(dead_code)]
  #[inline]
  pub fn new(val: i32) -> Self {
    TreeNode {
      val,
      left: None,
      right: None
    }
  }
}

struct Solution;

impl Solution {

    #[allow(dead_code)]
    pub fn add_one_row(root: Option<Rc<RefCell<TreeNode>>>, val: i32, depth: i32) -> Option<Rc<RefCell<TreeNode>>> {
        if depth == 1 {
            tree!(val, root, None)
        } else {
            Self::post_order(root, val, depth, 1)
        }
    }

    fn post_order(root: Option<Rc<RefCell<TreeNode>>>, val: i32, depth: i32, cur: i32) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(r) = root {
            let mut left = Self::post_order(r.borrow_mut().left.take(), val, depth, cur + 1);
            let mut right = Self::post_order(r.borrow_mut().right.take(), val, depth, cur + 1);

            if cur + 1 == depth {
                left = tree!(val, left, None);
                right = tree!(val, None, right);
            }

            tree!(r.borrow().val, left, right)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test{
    
}