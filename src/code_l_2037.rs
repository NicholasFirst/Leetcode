struct Solution;

impl Solution {
    // https://leetcode.cn/problems/minimum-number-of-moves-to-seat-everyone/
    #[allow(dead_code)]
    pub fn min_moves_to_seat(mut seats: Vec<i32>, mut students: Vec<i32>) -> i32 {
        seats.sort();
        students.sort();
        let mut ans = 0;
        for i in 0..seats.len() {
            ans += (seats[i] - students[i]).abs();
        }
        ans
    }
}

#[cfg(test)]
mod test{
    use super::Solution;

    #[test]
    fn test() {
        let seats = vec![2,2,6,6];
        let students = vec![1,3,2,6];
        let ans = Solution::min_moves_to_seat(seats, students);
        assert_eq!(ans, 4);
    }
}