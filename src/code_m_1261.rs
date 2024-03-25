// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
//
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use crate::node::tree_node::TreeNode;
use std::rc::Rc;
use std::cell::RefCell;


struct FindElements {
    node: TreeNode,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
#[allow(dead_code)]
impl FindElements {
    #[allow(dead_code)]
    fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        todo!()
    }
    
    fn find(&self, target: i32) -> bool {
        todo!()
    }
}
