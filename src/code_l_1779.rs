struct Solution;

impl Solution {
    // https://leetcode.cn/problems/find-nearest-point-that-has-the-same-x-or-y-coordinate/
    #[allow(dead_code)]
    pub fn nearest_valid_point(x: i32, y: i32, points: Vec<Vec<i32>>) -> i32 {
        let mut ans = (i32::MAX, -1);
        points.iter().enumerate().for_each(|(i, num)| {
            if num[0] == x || num[1] == y {
                let distance = (x - num[0]).abs() + (y - num[1]).abs();
                if distance < ans.0 {
                    ans.0 = distance;
                    ans.1 = i as i32;
                }
            }
        });
        ans.1
    }
}

#[cfg(test)]
mod test{
    use super::Solution;

    #[test]
    fn test() {
        let points: Vec<Vec<_>> = vec![[1,2].into(),[3,1].into(),[2,4].into(),[2,3].into(),[4,4].into()];
        let x = 3;
        let y = 4;

        let index = Solution::nearest_valid_point(x, y, points);
        assert_eq!(2, index);
    }

    #[test]
    fn test2() {
        let points: Vec<Vec<_>> = vec![[3,4].into()];
        let x = 3;
        let y = 4;

        let index = Solution::nearest_valid_point(x, y, points);
        assert_eq!(0, index);
    }

    #[test]
    fn test3() {
        let points: Vec<Vec<_>> = vec![[2,3].into()];
        let x = 3;
        let y = 4;

        let index = Solution::nearest_valid_point(x, y, points);
        assert_eq!(-1, index);
    }

    #[test]
    fn test4() {
        let r = 10.0;
        let p = std::f64::consts::PI;

        let len = 2.0 * p * r;
        let len = len.sin();
        println!("{len}");
    }
}