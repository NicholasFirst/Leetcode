struct Solution;

impl Solution {
    // https://leetcode.cn/problems/find-the-first-player-to-win-k-games-in-a-row/description/?envType=daily-question&envId=2024-10-24
    #[allow(dead_code)]
    pub fn find_winning_player(skills: Vec<i32>, k: i32) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3175() {
        assert_eq!(Solution::find_winning_player(vec![4,2,6,3,9], 2), 2);
        assert_eq!(Solution::find_winning_player(vec![2, 5, 4], 3), 1);
    }
}