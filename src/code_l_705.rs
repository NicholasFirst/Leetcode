struct MyHashSet {
    data: Vec<Vec<i32>>,
}

#[allow(dead_code)]
impl MyHashSet {
    const BASE: i32 = 769;

    fn new() -> Self {
        MyHashSet {
            data: vec![vec![]; Self::BASE as usize]
        }
    }
    
    fn hash(key: i32) -> usize {
        (key % Self::BASE) as usize
    }

    fn add(&mut self, key: i32) {
        let h = MyHashSet::hash(key);
        match self.data[h].binary_search(&key) {
            Err(i) => self.data[h].insert(i, key),
            _ => {} // already exists
        }
    }
    
    fn remove(&mut self, key: i32) {
        let h = MyHashSet::hash(key);
        match self.data[h].binary_search(&key) {
            Ok(idx) => { self.data[h].remove(idx); }
            _ => {}
        }
    }
    
    fn contains(&self, key: i32) -> bool {
        self.data[MyHashSet::hash(key)].binary_search(&key).is_ok()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let mut set = MyHashSet::new();
        set.add(1);
        set.add(2);
        assert_eq!(set.contains(1), true);
        assert_eq!(set.contains(3), false);
        set.add(2);
        assert_eq!(set.contains(2), true);
        set.remove(2);
        assert_eq!(set.contains(2), false);
        
    }
}