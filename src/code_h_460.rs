use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;

//构建节点
struct ListNode {
    key: i32,
    val: i32,
    freq: i32,
    prev: Option<Rc<RefCell<ListNode>>>,
    next: Option<Rc<RefCell<ListNode>>>,
}

impl ListNode{
    fn new(key: i32, val: i32) -> Self {
        Self {
            key,
            val,
            freq: 1,
            prev: None,
            next: None,
        }
    }
}

//构建双向链表
#[derive(Default)]
#[allow(dead_code)]
struct List{
    head: Option<Rc<RefCell<ListNode>>>,
    tail: Option<Rc<RefCell<ListNode>>>,
}

#[allow(dead_code)]
impl List {
    fn new() -> Self {
        Default::default()
    }

    fn pop(&mut self) -> Option<Rc<RefCell<ListNode>>> {
        if let Some(x) = self.head.take() {
            if let Some(y) = x.borrow_mut().next.take() {
                y.borrow_mut().prev = None;
                self.head = Some(y);
            } else {
                self.head = None;
                self.tail = None;
            }
            Some(x)
        } else {
            None
        }
    }

    fn push(&mut self, new_node: Option<Rc<RefCell<ListNode>>>) {
        if let Some(t) = self.tail.take() {
            if let Some(n) = &new_node {
                t.borrow_mut().next = Some(n.clone());
                n.borrow_mut().prev = Some(t);
            }
        }else {
            self.head = new_node.clone();
        }
        self.tail = new_node;
    }

    fn is_empty(&self) -> bool {
        self.head.is_none() && self.tail.is_none()
    }
}

#[allow(dead_code)]
struct LFUCache {
    cap: i32,
    used: i32,
    least_freq: i32,
    data: HashMap<i32, Rc<RefCell<ListNode>>>,
    freq: HashMap<i32, List>
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
#[allow(dead_code)]
impl LFUCache {
    fn new(capacity: i32) -> Self {
        Self {
            cap: capacity,
            used: 0,
            least_freq: 0,
            data: HashMap::new(),
            freq: HashMap::new(),
        }
    }
    

    fn get(&mut self, key: i32) -> i32 {
        if self.cap == 0 {
            return -1;
        }
        if let Some(node) = self.data.get(&key) {
            let val = node.borrow().val;
            let cloned = node.clone();
            self.update(cloned);
            val
        } else {
            -1
        }
    }
    

    fn put(&mut self, key: i32, value: i32) {
        if self.cap == 0 {
            return;
        }

        if let Some(node) = self.data.get(&key) {
            node.borrow_mut().val = value;
            let cloned = node.clone();
            self.update(cloned);
        } else {
            if self.used == self.cap {
                let node = if let Some(list) = self.freq.get_mut(&self.least_freq) {
                    list.pop()
                } else {
                    None
                };
                if let Some(n) = node {
                    self.data.remove(&n.borrow().key);
                }
            } else {
                self.used += 1;
            }

            let new_node = Rc::new(RefCell::new(ListNode::new(key, value)));
            self.least_freq = 1;
            self.data.insert(key, new_node.clone());
            self.freq.entry(1).or_default().push(Some(new_node));
        }
    }


    fn update(&mut self, node: Rc<RefCell<ListNode>>) {
        let freq = node.borrow().freq;
        node.borrow_mut().freq += 1;

        if let Some(list) = self.freq.get_mut(&freq) {
            let mut n = node.borrow_mut();
            match (n.prev.take(), n.next.take()) {
                (Some(prev), Some(next)) => {
                    next.borrow_mut().prev = Some(prev.clone());
                    prev.borrow_mut().next = Some(next);
                }
                (None, Some(next)) => {
                    next.borrow_mut().prev = None;
                    list.head = Some(next);
                }
                (Some(prev), None) => {
                    prev.borrow_mut().next = None;
                    list.tail = Some(prev);
                }
                (None, None) => {
                    list.head = None;
                    list.tail = None;
                }
            };
        }

        self.freq.entry(freq + 1).or_default().push(Some(node));

        if freq == self.least_freq && self.freq.entry(freq).or_default().is_empty() {
            self.least_freq += 1;
        }
    }
}

#[cfg(test)]
mod test{
    use super::LFUCache;

    #[test]
    fn test(){
        let mut obj = LFUCache::new(2);
        obj.put(1, 1);
        obj.put(2, 2);
        let ret_1: i32 = obj.get(1);
        assert_eq!(1, ret_1);
        obj.put(3, 3);
        let ret_2: i32 = obj.get(2);
        assert_eq!(-1, ret_2);
        let ret_3: i32 = obj.get(3);
        assert_eq!(3, ret_3);
        let ret_1_1 = obj.get(1);
        let ret_3_3 = obj.get(3);
        let ret_4_4 = obj.get(4);
        assert_eq!(ret_1_1, 1);
        assert_eq!(ret_3_3, 3);
        assert_eq!(-1, ret_4_4);
    }
}
