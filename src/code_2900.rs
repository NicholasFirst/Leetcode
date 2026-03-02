struct Solution;

impl Solution {

    // https://leetcode.cn/problems/longest-unequal-adjacent-groups-subsequence-i/?envType=daily-question&envId=2025-05-15
    #[allow(dead_code)]
    pub fn get_longest_subsequence(words: Vec<String>, groups: Vec<i32>) -> Vec<String> {
        let n = groups.len();
        let mut ans = vec![];
        for (i, word) in words.into_iter().enumerate() {
            if i == n - 1 || groups[i] != groups[i + 1] { // i 是连续相同段的末尾
                ans.push(word);
            }
        }
        ans
    }
}