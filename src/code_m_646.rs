struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn find_longest_chain(mut pairs: Vec<Vec<i32>>) -> i32 {
        pairs.sort_by(|a, b| a[1].cmp(&b[1]));
        pairs
            .iter()
            .enumerate()
            .fold((0, 1), |(i, cnt), (j, pair)| {
                if pairs[i][1] < pair[0] {
                    (j, cnt + 1)
                } else {
                    (i, cnt)
                }
            })
            .1
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test() {
        let pairs = vec![vec![1, 2], vec![2, 3], vec![3, 4]];
        let res = Solution::find_longest_chain(pairs);
        assert_eq!(res, 2);
    }
}
