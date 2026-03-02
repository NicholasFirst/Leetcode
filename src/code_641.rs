use std::collections::VecDeque;

/// 双端队列
/// https://leetcode.cn/problems/design-circular-deque/
#[allow(dead_code)]
struct MyCircularDeque {
    capacity: usize,
    queue: VecDeque<i32>,
}

impl MyCircularDeque {
    #[allow(dead_code)]
    fn new(k: i32) -> Self {
        MyCircularDeque {
            capacity: k as usize,
            queue: VecDeque::with_capacity(k as usize),
        }
    }

    #[allow(dead_code)]
    fn insert_front(&mut self, value: i32) -> bool {
        if !self.is_full() {
            self.queue.push_front(value);
            true
        } else {
            false
        }
    }

    #[allow(dead_code)]
    fn insert_last(&mut self, value: i32) -> bool {
        if !self.is_full() {
            self.queue.push_back(value);
            true
        } else {
            false
        }
    }

    #[allow(dead_code)]
    fn delete_front(&mut self) -> bool {
        if !self.is_empty() {
            self.queue.pop_front();
            true
        } else {
            false
        }
    }

    #[allow(dead_code)]
    fn delete_last(&mut self) -> bool {
        if !self.is_empty() {
            self.queue.pop_back();
            true
        } else {
            false
        }
    }

    #[allow(dead_code)]
    fn get_front(&self) -> i32 {
        *self.queue.front().unwrap_or(&-1)
    }

    #[allow(dead_code)]
    fn get_rear(&self) -> i32 {
        *self.queue.back().unwrap_or(&-1)
    }

    #[allow(dead_code)]
    fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }

    #[allow(dead_code)]
    fn is_full(&self) -> bool {
        self.queue.len() == self.capacity
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let k = 2;
        let mut obj = MyCircularDeque::new(k);
        let ret_1: bool = obj.is_empty();
        assert!(ret_1);
        let ret_2: bool = obj.insert_front(10);
        assert!(ret_2);
        let ret_3: bool = obj.insert_last(-10);
        assert!(ret_3);
        let ret_4: bool = obj.insert_front(0);
        assert!(!ret_4);
        let ret_5: bool = obj.is_full();
        assert!(ret_5);
        let ret_6: i32 = obj.get_rear();
        assert_eq!(ret_6, -10);
        let ret_7: i32 = obj.get_front();
        assert_eq!(ret_7, 10);
        let ret_8: bool = obj.delete_front();
        assert!(ret_8);
        let ret_9: bool = obj.delete_front();
        assert!(ret_9);
        let ret_10: bool = obj.delete_last();
        assert!(!ret_10);
        let ret_11: bool = obj.is_empty();
        assert!(ret_11);
    }
}
