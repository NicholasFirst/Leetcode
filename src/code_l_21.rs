use crate::list_node::ListNode;


struct Solution;

impl Solution{

    // https://leetcode.cn/problems/merge-two-sorted-lists/
    #[allow(dead_code)]
    pub fn merge_two_lists(list1: Option<Box<ListNode>>, list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        match (list1, list2) {
            (None, None) => None,
            (None, r) => r,
            (l, None) => l,
            (Some(mut l), Some(mut r)) => {
                if l.val <= r.val {
                    l.next = Self::merge_two_lists(l.next, Some(r));
                    Some(l)
                } else {
                    r.next = Self::merge_two_lists(Some(l), r.next);
                    Some(r)
                }
            }
        }
    }
}


#[cfg(test)]
mod test{
    use super::Solution;
    use crate::list_node::ListNode;

    #[test]
    fn test() {
        let l1 = vec![1,2,4];
        let l2 = vec![1,3,4];
        let list1 = ListNode::from(l1);
        let list2 = ListNode::from(l2);
        let result = Solution::merge_two_lists(Some(Box::new(list1)), Some(Box::new(list2)));
        let target = vec![1,1,2,3,4,4];
        let res = Some(Box::new(ListNode::from(target)));
        assert_eq!(res, result);
    }
}