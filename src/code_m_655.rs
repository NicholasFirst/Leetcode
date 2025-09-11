use crate::node::tree_node::TreeNode;
struct Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    /// https://leetcode.cn/problems/print-binary-tree/solution/rust-by-drackramoray-btaw/
    #[allow(dead_code)]
    pub fn print_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<String>> {
        let deep = Solution::get_deep(&root);
        let width = (1 << deep) - 1;
        let mut res = vec![vec!["".to_string(); width]; deep];
        let mut queue = vec![];
        if let Some(r) = root {
            queue.push((r, 0, (width - 1) / 2));
            while queue.len() > 0 {
                let mut tmp = vec![];
                for i in 0..queue.len() {
                    let (ref node, row, col) = queue[i];
                    res[row][col] = format!("{}", node.borrow().val);

                    if let Some(left) = node.borrow_mut().left.take() {
                        tmp.push((left, row + 1, col - (1 << (deep - row - 2))));
                    }

                    if let Some(right) = node.borrow_mut().right.take() {
                        tmp.push((right, row + 1, col + (1 << (deep - row - 2))));
                    }
                }
                queue = tmp;
            }
        }
        res
    }

    fn get_deep(root: &Option<Rc<RefCell<TreeNode>>>) -> usize {
        if let Some(r) = root {
            1 + Self::get_deep(&r.borrow().left).max(Self::get_deep(&r.borrow().right))
        } else {
            0
        }
    }
}

#[cfg(test)]
mod test {
    use super::Solution;
    use crate::node::tree_node::TreeNode;

    #[test]
    fn test() {
        let v = vec![Some(1), Some(2), Some(3), None, Some(4), None, None];
        let param = TreeNode::from(v).unwrap();
        let res = Solution::print_tree(param);
        let target = vec![
            vec![
                "".to_string(),
                "".to_string(),
                "".to_string(),
                "1".to_string(),
                "".to_string(),
                "".to_string(),
                "".to_string(),
            ],
            vec![
                "".to_string(),
                "2".to_string(),
                "".to_string(),
                "".to_string(),
                "".to_string(),
                "3".to_string(),
                "".to_string(),
            ],
            vec![
                "".to_string(),
                "".to_string(),
                "4".to_string(),
                "".to_string(),
                "".to_string(),
                "".to_string(),
                "".to_string(),
            ],
        ];
        assert_eq!(res, target);
    }
}
