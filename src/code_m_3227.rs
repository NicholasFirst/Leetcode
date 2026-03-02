struct Solution;

impl Solution {
    
    // https://leetcode.cn/problems/vowels-game-in-a-string/description/?envType=daily-question&envId=2025-09-12
    #[allow(dead_code)]
     pub fn does_alice_win(s: String) -> bool {
        s.chars().any(|c| matches!(c, 'a' | 'e' | 'i' | 'o' | 'u'))
    }
}


#[cfg(test)]
mod tests {
    
    use super::*;

    #[test]
    fn test_3227() {
        assert_eq!(Solution::does_alice_win("leetcoder".to_string()), true);
        assert_eq!(Solution::does_alice_win("bbcd".to_string()), false);
    }
}