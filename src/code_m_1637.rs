struct Solution;

impl Solution {
    // https://leetcode.cn/problems/widest-vertical-area-between-two-points-containing-no-points/
    #[allow(dead_code)]
    pub fn max_width_of_vertical_area(mut points: Vec<Vec<i32>>) -> i32 {
        points.sort_by(|x, y| x[0].cmp(&y[0]));
        (1..points.len()).fold(0, |ret, i| ret.max(points[i][0] - points[i - 1][0]))
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test() {
        let points = vec![vec![8, 7], vec![9, 9], vec![7, 4], vec![9, 7]];
        let ans = Solution::max_width_of_vertical_area(points);
        assert_eq!(1, ans);
    }

    #[test]
    fn test1() {
        let points = vec![vec![3,1],vec![9,0],vec![1,0],vec![1,4],vec![5,3],vec![8,8]];
        let ans = Solution::max_width_of_vertical_area(points);
        assert_eq!(3, ans);
    }
}
