struct Solution;

impl Solution {
    /// https://leetcode.cn/problems/number-of-students-doing-homework-at-a-given-time/
    #[allow(dead_code)]
    pub fn busy_student(start_time: Vec<i32>, end_time: Vec<i32>, query_time: i32) -> i32 {
        start_time.iter().zip(end_time.iter()).filter(|(&start, &end)| query_time >= start && query_time <= end).count() as i32
    }
}

#[cfg(test)]
mod test{
    use super::Solution;

    #[test]
    fn test() {
        let start_time = vec![1,2,3];
        let end_time = vec![3,2,7];
        let query_time = 4;
        let res = Solution::busy_student(start_time, end_time, query_time);
        assert_eq!(res, 1);
    }
}