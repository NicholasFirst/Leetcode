struct Solution;

impl Solution {

    // https://leetcode.cn/problems/russian-doll-envelopes/
    #[allow(dead_code)]
    pub fn max_envelopes(mut envelopes: Vec<Vec<i32>>) -> i32 {
        envelopes.sort_unstable_by(|a, b| a[0].cmp(&b[0]).then(b[1].cmp(&a[1])));
        let mut sub = vec![];
        for envelope in envelopes {
            let (_, h) = (envelope[0], envelope[1]);
            let i = sub.binary_search(&h);
            let i = match i {
                Ok(i) => i,
                Err(i) => i,
            };
            if i == sub.len() {
                sub.push(h);
            } else {
                sub[i] = h;
            }
        }
        sub.len() as i32
    }
}

#[cfg(test)]
mod test{
    use super::Solution;

    #[test]
    fn test() {
        let envelopes: Vec<Vec<i32>> = vec![vec![5,4],vec![6,4],vec![6,7],vec![2,3]];
        let ans = Solution::max_envelopes(envelopes);
        assert_eq!(ans, 3);
    }

    #[test]
    fn test1() {
        let envelopes: Vec<Vec<i32>> = vec![vec![1,1],vec![1,1],vec![1,1],vec![1,1]];
        let ans = Solution::max_envelopes(envelopes);
        assert_eq!(ans, 1);
    }

    #[test]
    fn test2() {
        let envelopes: Vec<Vec<i32>> = vec![vec![46,89],vec![50,53],vec![52,68],vec![72,45],vec![77,81]];
        let ans = Solution::max_envelopes(envelopes);
        assert_eq!(ans, 3);
    }
}