use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
struct Packet(i32, i32, i32);
struct Router {
    queue: VecDeque<Packet>,
    duplicate: HashSet<Packet>,
    query: HashMap<i32, VecDeque<i32>>,
}

#[allow(dead_code)]
impl Router {
    fn new(memory_limit: i32) -> Self {
        Self {
            queue: VecDeque::with_capacity(memory_limit as usize),
            duplicate: HashSet::with_capacity(memory_limit as usize),
            query: Default::default(),
        }
    }

    fn add_packet(&mut self, source: i32, destination: i32, timestamp: i32) -> bool {
        let packet = Packet(source, destination, timestamp);
        if self.duplicate.contains(&packet) {
            return false;
        }
        if self.queue.len() == self.queue.capacity() {
            self.forward_packet();
        }
        self.queue.push_back(packet);
        self.duplicate.insert(packet);
        self.query
            .entry(destination)
            .or_default()
            .push_back(timestamp);
        true
    }

    fn forward_packet(&mut self) -> Vec<i32> {
        let Some(p) = self.queue.pop_front() else {
            return Vec::new();
        };
        self.duplicate.remove(&p);
        self.query.get_mut(&p.1).unwrap().pop_front();
        vec![p.0, p.1, p.2]
    }

    
    fn get_count(&self, destination: i32, start_time: i32, end_time: i32) -> i32 {
        if let Some(v) = self.query.get(&destination) {
            let left = v.partition_point(|x| *x < start_time);
            let right = v.partition_point(|x| *x <= end_time);
            (right - left) as _
        } else {
            0
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    //["Router","addPacket","getCount"]
    // [[3],[1,4,6],[4,1,4]]
    #[test]
    fn test_1() {
        let mut router = Router::new(3);
        assert_eq!(router.add_packet(1, 4, 6), true);
        assert_eq!(router.get_count(4, 1, 4), 0);
    }

    //["Router","addPacket","addPacket","addPacket","addPacket","addPacket","forwardPacket","addPacket","getCount"]
    // [[3],[1,4,90],[2,5,90],[1,4,90],[3,5,95],[4,5,105],[],[5,2,110],[5,100,110]]
    #[test]
    fn test_2() {
        let mut router = Router::new(3);
        assert_eq!(router.add_packet(1, 4, 90), true);
        assert_eq!(router.add_packet(2, 5, 90), true);
        assert_eq!(router.add_packet(1, 4, 90), false);
        assert_eq!(router.add_packet(3, 5, 95), true);
        assert_eq!(router.add_packet(4, 5, 105), true);
        assert_eq!(router.forward_packet(), vec![2, 5, 90]);
        assert_eq!(router.add_packet(5, 2, 110), true);
        assert_eq!(router.get_count(5, 100, 110), 1);
    }
}



