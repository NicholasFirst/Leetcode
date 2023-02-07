struct Solution;

use std::collections::HashMap;

impl Solution {

    // https://leetcode.cn/problems/check-array-formation-through-concatenation/
    #[allow(dead_code)]
    pub fn can_form_array(arr: Vec<i32>, mut pieces: Vec<Vec<i32>>) -> bool {
        let indices = arr
            .iter()
            .enumerate()
            .map(|(i, n)| (n, i))
            .collect::<HashMap<_, _>>();
        pieces.sort_unstable_by_key(|piece| indices.get(&piece[0]));
        arr == pieces.concat()
    }
}

#[cfg(test)]
mod test {

    use super::Solution;

    #[test]
    fn test() {
        let arr = vec![91,4,64,78];
        let pieces = vec![vec![78], vec![4, 64], vec![91]];
        let res = Solution::can_form_array(arr, pieces);
        assert!(res);
    }

    #[test]
    fn test1() {
        let arr = vec![49,18,16];
        let pieces = vec![vec![16,18,49]];
        let res = Solution::can_form_array(arr, pieces);
        assert!(!res);
    }
}