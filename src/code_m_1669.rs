struct Solution;

use crate::list_node::ListNode;

impl Solution {

    // https://leetcode.cn/problems/merge-in-between-linked-lists/
    #[allow(dead_code)]
    pub fn merge_in_between(mut list1: Option<Box<ListNode>>, a: i32, b: i32, mut list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut start = &mut list1;
        for _ in 1..a {
            start = &mut start.as_deref_mut().unwrap().next;
        }

        let mut end = &mut start.clone();
        for _ in a - 2..b {
            end = &mut end.as_deref_mut().unwrap().next;
        }
        std::mem::swap(&mut start.as_deref_mut().unwrap().next, &mut list2);

        while start.as_ref().unwrap().next.is_some() {
            start = &mut start.as_deref_mut().unwrap().next;
        }
        std::mem::swap(&mut start.as_deref_mut().unwrap().next, &mut end);
        list1
    }
}


#[cfg(test)]
mod test{
    use super::Solution;

    #[test]
    fn test() {
        let list1 = vec![0,1,2,3,4,5,6]; 
        let a = 2; 
        let b = 5; 
        let list2 = vec![1000000,1000001,1000002,1000003,1000004];
        let list1 = Some(Box::new(list1.into()));
        let list2 = Some(Box::new(list2.into()));
        let ans = Solution::merge_in_between(list1, a, b, list2);
        dbg!(ans);
    }
}