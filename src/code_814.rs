use std::cell::RefCell;
use std::rc::Rc;
use crate::{Error, Result};

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode { val, left: None, right: None }
    }
}

impl std::convert::TryFrom<&[Option<i32>]> for TreeNode{
    type Error = Error;

    fn try_from(value: &[Option<i32>]) -> Result<Self> {
        for x in value {
            if let Some(val) = x {
                TreeNode::new(*val);
            }
        };
        todo!()
    }
}