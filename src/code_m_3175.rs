struct Solution;

impl Solution {
    // https://leetcode.cn/problems/find-the-first-player-to-win-k-games-in-a-row/description/?envType=daily-question&envId=2024-10-24
    // 官方题解
    #[allow(dead_code)]
    pub fn find_winning_player(skills: Vec<i32>, k: i32) -> i32 {
        let n = skills.len();
        let mut cnt = 0;
        let mut i = 0;
        let mut last_i = 0;

        while i < n {
            let mut j = i + 1;
            while j < n && skills[j] < skills[i] && cnt < k {
                j += 1;
                cnt += 1;
            }
            if cnt == k {
                return i as i32;
            }
            cnt = 1;
            last_i = i as i32;
            i = j;
        }
        last_i 
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