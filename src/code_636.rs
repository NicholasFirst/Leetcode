
struct Solution;

impl Solution {

    /// 
    #[allow(dead_code)]
    pub fn exclusive_time(n: i32, logs: Vec<String>) -> Vec<i32> {
        let mut ans = vec![0; n as usize];
        let mut stack = Vec::with_capacity(logs.len() >> 1);
        for log in logs {
            let log: Vec<_> = log.split(':').collect();
            let idx: usize = log[0].parse().unwrap();
            let time: i32 = log[2].parse().unwrap();
            if log[1] == "start" {
                stack.push((idx, time, 0));
            }else {
                let (idx, start, other) = stack.pop().unwrap();
                ans[idx] += time - start - other + 1;
                if let Some(t) = stack.last_mut() {
                    t.2 += time - start + 1;
                }
            }
        }
        ans
    }
}

#[cfg(test)]
mod test{
    use super::Solution;

    #[test]
    fn test() {
        let logs = vec!["0:start:0".to_owned(),
            "0:start:2".to_owned(),
            "0:end:5".to_owned(),
            "1:start:6".to_owned(),
            "1:end:6".to_owned(),
            "0:end:7".to_owned()];
        let res = Solution::exclusive_time(2, logs);
        assert_eq!(vec![7, 1], res);
    }

}