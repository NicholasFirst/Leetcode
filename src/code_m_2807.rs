use super::list_node::ListNode;

struct Solution;

impl Solution {
    #[allow(dead_code)]
    // https://leetcode.cn/problems/insert-greatest-common-divisors-in-linked-list/description/?envType=daily-question&envId=Invalid%20Date
    pub fn insert_greatest_common_divisors(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        
        let mut cur = &mut head;
        while cur.as_ref().unwrap().next.is_some() {
            let x = cur.as_mut().unwrap();
            let next = x.next.take();
            x.next = Some(Box::new(ListNode {
                val: Self::gcd(x.val, next.as_ref().unwrap().val),
                next,
            }));
            cur = &mut cur.as_mut().unwrap().next.as_mut().unwrap().next;
        }
        head
        // 实现代码
        
    }

    fn gcd(mut a: i32, mut b: i32) -> i32{
        while a != 0 {
            (a, b) = (b % a, a);
        }
        b
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::list_node::ListNode;

    #[test]
    fn test_insert_greatest_common_divisors() {
        let list = vec![18, 6, 10, 3];
        let list_node = ListNode::from(list);
        let param = Some(Box::new(list_node));
        let res = Solution::insert_greatest_common_divisors(param);
        let res_list = vec![18, 6, 6, 2, 10, 1, 3];
        let res_list_node = ListNode::from(res_list);
        let res_param = Some(Box::new(res_list_node));
        assert_eq!(res_param, res);
    }
}
