
use std::collections::HashMap;

struct AuthenticationManager {
    time_to_live: i32,
    map: HashMap<String, i32>,
}

impl AuthenticationManager {
    
    fn new(time_to_live: i32) -> Self {
        AuthenticationManager { time_to_live: time_to_live, map: HashMap::new() }
    }
    
    fn generate(&mut self, token_id: String, current_time: i32) {
        self.map.insert(token_id, current_time + self.time_to_live);
    }
    
    fn renew(&mut self, token_id: String, current_time: i32) {
        if let Some(s) = self.map.get(&token_id) {
            if s > &current_time {
                self.map.insert(token_id, current_time + self.time_to_live);
            }
        }
    }
    
    fn count_unexpired_tokens(&self, current_time: i32) -> i32 {
        self.map.iter().filter(|(_, v)| {
             **v > current_time
        }).count() as i32
    }
}


#[cfg(test)]
mod test{
    use super::AuthenticationManager;

    #[test]
    fn test() {
        let mut obj = AuthenticationManager::new(100);
        let token_id = "".to_string();
        let current_time = 112233;
        obj.generate(token_id.clone(), current_time);
        obj.renew(token_id.clone(), current_time);
        let ret_3 = obj.count_unexpired_tokens(current_time);
    }

    #[test]
    fn test1() {
        use std::time::{SystemTime, UNIX_EPOCH};

        let start = SystemTime::now();
        let sine = start.duration_since(UNIX_EPOCH).expect("ERROR!!");
        let sec = sine.as_secs();
        dbg!(sec);
    }
}