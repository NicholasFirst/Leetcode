struct Solution;

impl Solution {

    // https://leetcode.cn/problems/alert-using-same-key-card-three-or-more-times-in-a-one-hour-period/
    #[allow(dead_code)]
    pub fn alert_names(key_name: Vec<String>, key_time: Vec<String>) -> Vec<String> {
        fn to_min(time: &String) -> i32 {
            let time: Vec<&str> = time.split(':').collect();
            let (a,b):(_,_) = (time[0].parse::<i32>().unwrap(),time[1].parse::<i32>().unwrap());
            a * 60 + b
        }
        use std::collections::HashMap;

        let mut map = HashMap::new();
        let n = key_name.len();

        for i in 0..n{
            let count = map.entry(&key_name[i]).or_insert(Vec::new());
            count.push(to_min(&key_time[i]));
        }

        let mut ans:Vec<String> = Vec::new();
        for (k,v) in map.iter_mut(){
            v.sort_unstable();
            let vn = v.len();
            if vn<3 {continue;}
            for i in 0..vn-2{
                if v[i+2]-v[i] <= 60 {ans.push((*k).clone()); break;}
            }
        }
        ans.sort_unstable();
        ans
    }
}

#[cfg(test)]
mod test{
    use super::Solution;

    #[test]
    fn test() {
        let key_name = vec!["daniel".to_string(),"daniel".to_string(),"daniel".to_string(),"luis".to_string(),"luis".to_string(),"luis".to_string(),"luis".to_string()];
        let key_time = vec!["10:00".to_string(),"10:40".to_string(),"11:00".to_string(),"09:00".to_string(),"11:00".to_string(),"13:00".to_string(),"15:00".to_string()];
        let ans = Solution::alert_names(key_name, key_time);
        assert_eq!(vec!["daniel".to_string()], ans);
    }
}