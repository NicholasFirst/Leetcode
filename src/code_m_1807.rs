struct Solution;

impl Solution {
 
    // https://leetcode.cn/problems/evaluate-the-bracket-pairs-of-a-string/
    #[allow(dead_code)]
    pub fn evaluate(s: String, knowledge: Vec<Vec<String>>) -> String {
        // // 以下为DFS
        use std::collections::HashMap;
        let mut map = HashMap::with_capacity(knowledge.len());
        knowledge.into_iter().for_each(|v| {
            map.insert(v[0].clone(), v[1].clone());
        });
        let mut key = "".to_string();
        let mut add_key = false;
        let mut res = "".to_string();
        let q = "?".to_string();
        s.chars().for_each(|x| {
            match x {
                '(' => {
                    add_key = true;
                },
                ')' => {
                    add_key = false;
                    res.push_str(map.get(&key).unwrap_or(&q));
                    key.clear();
                },
                _ => {
                    if add_key {
                        key.push(x);
                    } else {
                        res.push(x);
                    }
                }
            }
        });
        res
    }
}

#[cfg(test)]
mod test{
    use super::Solution;

    #[test]
    fn test() {
        let s = "(name)is(age)yearsold".to_string();
        let knowledge = vec![vec!["name".to_string(),"bob".to_string()],vec!["age".to_string(),"two".to_string()]];
        let res = Solution::evaluate(s, knowledge);
        assert_eq!(res, "bobistwoyearsold".to_string())
    }
}