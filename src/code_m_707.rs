
struct MyLinkedList {
    head: Option<Box<Node>>
}

#[derive(Default)]
struct Node {
    val: i32,
    next: Option<Box<Node>>,
}

impl MyLinkedList {
    fn new() -> Self {
        MyLinkedList { head: Some(Box::new(Node::default())) }
    }

    fn get(&self, index: i32) -> i32 {
        todo!()
    }

    fn add_at_head(&mut self, val: i32) {
        self.head = Some(Box::new(Node {
            val, next: self.head.take()
        }))
    }

    fn add_at_tail(&mut self, val: i32) {
        let tail = Node {val, next: None};
        
    }

    fn add_at_index(&mut self, index: i32, val: i32) {
        todo!()
    }

    fn delete_at_index(&mut self, index: i32) {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use super::MyLinkedList;

    #[test]
    fn test() {
        let obj = MyLinkedList::new();
        // let ret_1: i32 = obj.get(index);
        // obj.add_at_head(val);
        // obj.add_at_tail(val);
        // obj.add_at_index(index, val);
        // obj.delete_at_index(index);
        // println!("{:?}", obj);
    }
}
