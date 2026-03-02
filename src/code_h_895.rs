use std::collections::HashMap;

struct FreqStack {
    freqs: HashMap<i32, usize>,
    stacks: Vec<Vec<i32>>,
}

impl FreqStack {

    #[allow(dead_code)]
    fn new() -> Self {
        FreqStack {
            freqs: HashMap::new(),
            stacks: Vec::new(),
        }
    }
    
    #[allow(dead_code)]
    fn push(&mut self, val: i32) {
        self.freqs.entry(val).and_modify(|f|*f+=1).or_insert(0);
        let freq = self.freqs[&val];
        if freq >= self.stacks.len() {
            self.stacks.push(Vec::new());
        }
        self.stacks[freq].push(val);
    }
    
    #[allow(dead_code)]
    fn pop(&mut self) -> i32 {
        let last_stack = self.stacks.last_mut().unwrap();
        let val = last_stack.pop().unwrap();
        if last_stack.is_empty() {
            self.stacks.pop();
        }
        if *self.freqs.get_mut(&val).unwrap() > 0 {
            *self.freqs.get_mut(&val).unwrap() -= 1;
        }
        return val;
    }
}



#[cfg(test)]
mod test{
    use super::FreqStack;

    #[test]
    fn test() {
        let mut s = FreqStack::new();
        s.push(4);
        s.push(0);
        s.push(9);
        s.push(3);
        s.push(4);
        s.push(2);
        assert_eq!(s.pop(), 4);
        s.push(6);
        assert_eq!(s.pop(), 6);
        s.push(1);
        assert_eq!(s.pop(), 1);
        s.push(1);
        assert_eq!(s.pop(), 1);
        s.push(4);
        assert_eq!(s.pop(), 4);
        assert_eq!(s.pop(), 2);
        assert_eq!(s.pop(), 3);
        assert_eq!(s.pop(), 9);
        assert_eq!(s.pop(), 0);
        assert_eq!(s.pop(), 4);
    }
}