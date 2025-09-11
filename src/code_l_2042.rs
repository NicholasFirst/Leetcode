struct Solution;

impl Solution {

    // https://leetcode.cn/problems/check-if-numbers-are-ascending-in-a-sentence/
    #[allow(dead_code)]
    pub fn are_numbers_ascending(s: String) -> bool {
        let mut last = i32::MIN;
        let mut ans = true;
        s.split_whitespace().filter_map(|x| {x.parse::<i32>().ok()}).for_each(|x| {
            if ans {
                if last < x {
                    last = x;
                }else{
                    ans = false;
                }
            }
        });
        ans
    }
}

#[cfg(test)]
mod test{
    use super::Solution;

    #[test]
    fn test() {
        let s = "1 box has 3 blue 4 red 6 green and 12 yellow marbles".to_string();
        let ans = Solution::are_numbers_ascending(s);
        assert!(ans);
    }
}