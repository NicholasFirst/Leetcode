struct Solution;

impl Solution {
    
    // https://leetcode.cn/problems/best-poker-hand/description/
    #[allow(dead_code)]
    pub fn best_hand(mut ranks: Vec<i32>, suits: Vec<char>) -> String {
        if suits.iter().all(|&x|x==suits[0]) {
            "Flush"
        } else {
            ranks.sort_unstable();
            let c=ranks.windows(2).fold((0,0),|mut s,x|{if x[0]==x[1] {s.1+=1} else {s.0=s.0.max(s.1);s.1=0};s});
            match c.0.max(c.1) {
                0=>"High Card",
                1=>"Pair"
                ,_=>"Three of a Kind"
            }
        }.to_string()
    }
}


#[cfg(test)]
mod test{
    use super::Solution;

    #[test]
    fn test() {
        let ranks = vec![13,2,3,1,9];
        let suits = vec!['a','a','a','a','a'];
        let res = Solution::best_hand(ranks, suits);
        assert_eq!(res, "Flush".to_string());
    }
}