pub mod tree_node{
    use std::rc::Rc;
    use std::cell::RefCell;
    use crate::Result;
    

    type NODE = Option<Rc<RefCell<TreeNode>>>;

    #[derive(Debug, PartialEq, Eq)]
    pub struct TreeNode {
        pub val: i32,
        pub left: NODE,
        pub right: NODE,
    }

    impl TreeNode {
        #[allow(dead_code)]
        #[inline]
        pub fn new(val: i32) -> Self {
            TreeNode { val, left: None, right: None }
        }

        fn create_node(arr: &Vec<Option<i32>>, i: usize) -> Option<Rc<RefCell<TreeNode>>> {
            if i > arr.len() {
                return None;
            }
            let root = if let Some(r) = arr[i - 1] {
                Some(Rc::new(RefCell::new(TreeNode {
                val: r,
                left: TreeNode::create_node(arr, i * 2),
                right: TreeNode::create_node(arr, i * 2 + 1),
            })))}else {
                None
            };
            root
        }

        /// 数组生成二叉树, 数组需满足长度 `2^n + 1`
        #[allow(dead_code)]
        pub fn from(value: Vec<Option<i32>>) -> Result<Option<Rc<RefCell<TreeNode>>>> {
            if (value.len() + 1) % 2 != 0 {
                return Err("invalid list length!".into());
            }
            Ok(TreeNode::create_node(&value, 1))
        }

        /// https://leetcode.cn/problems/maximum-binary-tree/
        /// 该方法递归生成最大二叉树, 以下为题目描述
        /// 创建一个根节点，其值为 nums 中的最大值。
        /// 递归地在最大值 左边 的 子数组前缀上 构建左子树。
        /// 递归地在最大值 右边 的 子数组后缀上 构建右子树。
        pub fn construct_maximum_binary_tree(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
            if nums.len() == 0 {
                return Option::None;
            }
            let mut i = 0;
            for j in 0..nums.len() {
                if nums[j] > nums[i] {
                    i = j;
                }
            }
            let left = Self::construct_maximum_binary_tree(nums[..i].to_vec());
            let right = Self::construct_maximum_binary_tree(nums[i + 1..].to_vec());
            Some(Rc::new(RefCell::new(TreeNode {
                val: nums[i],
                left,
                right,
            })))
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_node_from() {
        let target = vec![
            Some(1), Some(2), Some(3), Some(4),
            Some(5), None, Some(6), Some(7),
            None, None,None,None,None,None,Some(8)
        ];
        let res = tree_node::TreeNode::from(target);
        println!("{:?}", res);
    }
}