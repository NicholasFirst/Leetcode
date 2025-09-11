// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

impl From<Vec<i32>> for ListNode {
    fn from(value: Vec<i32>) -> Self {
        if value.len() == 0 {
            panic!("Can not use empty vec to ListNode!!!");
        }
        let mut list = value.into_iter().map(|v| ListNode::new(v)).collect::<Vec<ListNode>>();
        let mut head = list.pop().unwrap();
        while let Some(mut node) = list.pop() {
            node.next = Some(Box::new(head));
            head = node;
        };
        head
    }
}


#[cfg(test)]
mod test{
    use super::ListNode;

    #[test]
    fn test() {
        let vec = vec![1,2,3,4,5,6,7,8,9];
        let list_node: ListNode = vec.into();
        dbg!(list_node);
    }
}