use crate::node::tree_node::TreeNode;
struct Solution;

use std::rc::Rc;
use std::cell::RefCell;
impl Solution{

    /// https://leetcode.cn/problems/deepest-leaves-sum/
    /// 广度优先算法
    #[allow(dead_code)]
    pub fn deepest_leaves_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut ans = 0;
        let mut queue = vec![];
        if let Some(r) = root {
            queue.push(r);
            while queue.len() > 0 {
                let mut tmp = vec![];
                ans = 0;

                for i in 0..queue.len() {
                    ans += queue[i].borrow().val;
                    if let Some(left) = queue[i].borrow_mut().left.take() {
                        tmp.push(left);
                    }
                    if let Some(right) = queue[i].borrow_mut().right.take() {
                        tmp.push(right);
                    }
                }
                queue = tmp;
            }
        }
        ans
    }
}

#[cfg(test)]
mod test{
    use crate::node::tree_node::TreeNode;
    use super::Solution;

    #[test]
    fn test() {
        let target = vec![
            Some(1), Some(2), Some(3), Some(4),
            Some(5), None, Some(6), Some(7),
            None, None,None,None,None,None,Some(8)
        ];
        let root = TreeNode::from(target);
        let ans = Solution::deepest_leaves_sum(root.unwrap());
        assert_eq!(ans, 15);
    }

    /// for 循环中使用外部数组
    /// 在 for 循环中,vec.len()只会在循环开始始之时进行求值，之后就一直使用该值
    /// 不会变化
    #[test]
    fn test_for() {
        let mut vec =  vec![1, 2, 3];
        for i in 0..vec.len() {
            vec.push(i);            
        }
        assert_eq!(vec![1, 2, 3, 0, 1, 2], vec);
    }

    /// while循环，该循环与for相反，每次都会重新求值
    #[test]
    #[should_panic]
    fn test_while() {
        let mut vec =  vec![1, 2, 3];
        let mut i = 0;
        while i < vec.len() {
            vec.push(i);
            i += 1;
        }
    }
}