use std::collections::{BTreeMap, HashMap};

struct StockPrice {
    timestamp_to_price: HashMap<i32, i32>,
    price_to_cnt: BTreeMap<i32, i32>,
    max_timestamp: i32,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
#[allow(dead_code)]
impl StockPrice {
    fn new() -> Self {
        StockPrice {
            timestamp_to_price: HashMap::new(),
            price_to_cnt: BTreeMap::new(),
            max_timestamp: 0,
        }
    }

    fn update(&mut self, timestamp: i32, price: i32) {
        self.max_timestamp = self.max_timestamp.max(timestamp);
        if self.timestamp_to_price.contains_key(&timestamp) {
            if let Some(prev_price) = self.timestamp_to_price.get(&timestamp) {
                if let Some(prev_cnt) = self.price_to_cnt.get(prev_price) {
                    if *prev_cnt == 1 { self.price_to_cnt.remove(&prev_price); } else { self.price_to_cnt.insert(*prev_price, *prev_cnt - 1); }
                }
            }
        }
        self.timestamp_to_price.insert(timestamp, price);
        *self.price_to_cnt.entry(price).or_insert(0) += 1;
    }

    fn current(&self) -> i32 {
        *self.timestamp_to_price.get(&self.max_timestamp).unwrap()
    }

    fn maximum(&self) -> i32 {
        *self.price_to_cnt.iter().rev().next().unwrap().0
    }

    fn minimum(&self) -> i32 {
        *self.price_to_cnt.iter().next().unwrap().0
    }
}


/**
 * Your StockPrice object will be instantiated and called as such:
 * let obj = StockPrice::new();
 * obj.update(timestamp, price);
 * let ret_2: i32 = obj.current();
 * let ret_3: i32 = obj.maximum();
 * let ret_4: i32 = obj.minimum();
 */

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let mut obj = StockPrice::new();
        obj.update(1, 10);
        obj.update(2, 5);
        let ret_1: i32 = obj.current();
        let ret_2: i32 = obj.maximum();
        obj.update(1, 3);
        let ret_3: i32 = obj.maximum();
        obj.update(4, 2);
        let ret_4: i32 = obj.minimum();
        assert_eq!(ret_1, 5);
        assert_eq!(ret_2, 10);
        assert_eq!(ret_3, 5);
        assert_eq!(ret_4, 2);
    }
}
