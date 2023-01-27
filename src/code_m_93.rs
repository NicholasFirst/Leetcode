struct Solution;

impl Solution {
    
    // https://leetcode.cn/problems/restore-ip-addresses/
    #[allow(dead_code)]
    pub fn restore_ip_addresses(s: String) -> Vec<String> {
        let mut result = vec![];
        let mut cur = vec![];
        trace(s.as_str(), &mut cur, 0, &mut result);
        dbg!(&result);
        result
    }
}

fn trace(s: &str, cur: &mut Vec<String>, pos: usize, result: &mut Vec<String>) {
    if cur.len() == 4 {
        if pos == s.len() {
            result.push(cur.join("."));
        }
        return
    }

    for i in 1..4 {
        if pos + i > s.len() {
            break
        }
        let seg = &s[pos..pos + i];
        let val = seg.parse::<i32>().unwrap();
        if seg.starts_with("0") && seg.len() > 1 || val > 255 {
            continue
        }
        cur.push(seg.to_string());
        trace(s, cur, pos + i, result);
        cur.pop();
    }
}


#[cfg(test)]
mod test{
    use super::Solution;

    #[test]
    fn test() {
        let s = "25525511135".to_string();
        let res = Solution::restore_ip_addresses(s);
        assert!(res.contains(&"255.255.11.135".to_string()));
        assert!(res.contains(&"255.255.111.35".to_string()));
    }

    #[test]
    fn test1() {
        let s = "101023".to_string();
        let res = Solution::restore_ip_addresses(s);
        assert!(res.contains(&"1.0.10.23".to_string()));
        assert!(res.contains(&"1.0.102.3".to_string()));
        assert!(res.contains(&"10.1.0.23".to_string()));
        assert!(res.contains(&"10.10.2.3".to_string()));
        assert!(res.contains(&"101.0.2.3".to_string()));
    }
}