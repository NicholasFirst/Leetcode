struct Solution;

impl Solution {

    /// https://leetcode.cn/problems/merge-intervals/
    #[allow(dead_code)]
    pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        if intervals.len() <= 1 {
            return intervals;
        }
        let mut intervals = intervals;
        intervals.sort_by(|a, b| a.cmp(&b));
        let mut res = vec![];
        let mut curr = intervals[0].to_owned();
        for i in 1..intervals.len() {
            if curr[1] < intervals[i][0] {
                res.push(curr.to_owned());
                curr = intervals[i].to_owned();
                continue;
            }else if curr[1] >= intervals[i][1] {
                continue;
            }else {
                curr[1] = intervals[i][1];
            }
        }
        res.push(curr);
        res
    }
}

#[cfg(test)]
mod test{
    use super::Solution;

    #[test]
    fn test1 () {
        let intervals = vec![vec![1,3],vec![2,6],vec![8,10],vec![15,18]];
        let res = Solution::merge(intervals);
        assert_eq!(vec![vec![1,6],vec![8,10],vec![15,18]], res)
    }

    #[test]
    fn test2() {
        let intervals = vec![vec![1,4],vec![4,5]];
        let res = Solution::merge(intervals);
        assert_eq!(vec![vec![1,5]], res)
    }
}