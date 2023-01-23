struct Solution;

impl Solution {

    //https://leetcode.cn/problems/sender-with-largest-word-count/description/
    #[allow(dead_code)]
    pub fn largest_word_count(messages: Vec<String>, senders: Vec<String>) -> String {
        use std::cmp::Ordering;
        use std::collections::HashMap;
        let mut map = HashMap::new();

        messages.into_iter().zip(senders.into_iter()).for_each(
            |(m,s)|
            *map.entry(s).or_insert(0) += m.split_ascii_whitespace().count()
        );

        let v = map.into_iter().collect::<Vec<(String,usize)>>();
        
        v.into_iter().max_by(
            |x,y| 
            match x.1.cmp(&y.1){
                Ordering::Equal => x.0.cmp(&y.0),
                res => res,
            }
        ).unwrap().0

    }
}

#[cfg(test)]
mod test{
    use super::Solution;

    #[test]
    fn test() {
        let messages = vec![
            "Hello userTwooo".to_string(), 
            "Hi userThree".to_string(), 
            "Wonderful day Alice".to_string(), 
            "Nice day userThree".to_string()
        ]; 
        let senders = vec!["Alice".to_string(),"userTwo".to_string(),"userThree".to_string(),"Alice".to_string()];
        let res = Solution::largest_word_count(messages, senders);
        assert_eq!(res, "Alice".to_string());
    }


    #[test]
    fn test1() {
        let messages = vec![
            "How is leetcode for everyone".to_owned(),"Leetcode is useful for practice".to_owned()
        ]; 
        let senders = vec!["Bob".to_owned(),"Charlie".to_owned()];
        let res = Solution::largest_word_count(messages, senders);
        assert_eq!(res, "Charlie".to_string());
    }
}