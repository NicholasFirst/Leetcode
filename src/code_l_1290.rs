struct Solution; 
use crate::list_node::ListNode;

impl Solution {
    // https://leetcode.cn/problems/convert-binary-number-in-a-linked-list-to-integer/?envType=daily-question&envId=2025-07-14
    #[allow(dead_code)]
    pub fn get_decimal_value(head: Option<Box<ListNode>>) -> i32 {
        let mut result = 0;
        let mut current = head;
        
        while let Some(node) = current {
            result = (result << 1) | node.val; // 左移一位并加上当前节点的值
            current = node.next; // 移动到下一个节点
        }
        
        result
    }
}


#[cfg(test)]
mod tests {
    
    use super::*;

    #[test]
    fn test() {
        let list = vec![1, 0, 1];
        let list_node: ListNode = list.into();
        let head = Some(Box::new(list_node));
        assert_eq!(Solution::get_decimal_value(head), 5);
    }
    #[test]
    fn test_1() {
        let list = vec![0];
        let list_node: ListNode = list.into();
        let head = Some(Box::new(list_node));
        assert_eq!(Solution::get_decimal_value(head), 0);
    }

    #[test]
    fn test_2() {
        let list = vec![0,1,0,1];
        let list_node: ListNode = list.into();
        let head = Some(Box::new(list_node));
        assert_eq!(Solution::get_decimal_value(head), 5);
    }
}
