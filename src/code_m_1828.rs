struct Solution;

impl Solution {

    // https://leetcode.cn/problems/queries-on-number-of-points-inside-a-circle/
    #[allow(dead_code)]
    pub fn count_points(points: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        queries.iter().map(|c| {
            let (cx, cy, cr) = (c[0], c[1], c[2]);
            points.iter().filter(|&point| {
                let (px, py) = (point[0], point[1]);
                (px - cx) * (px - cx) + (py - cy) * (py - cy) <= cr * cr
            }).count() as i32
        }).collect()
    }
}


#[cfg(test)]
mod test{
    use super::Solution;

    #[test]
    fn test() {
        let points = vec![vec![1,1],vec![2,2],vec![3,3],vec![4,4],vec![5,5]]; 
        let queries = vec![vec![1,2,2],vec![2,2,2],vec![4,3,2],vec![4,3,3]];
        let ans = Solution::count_points(points, queries);
        assert_eq!(ans, vec![2,3,2,4]);
    }
}