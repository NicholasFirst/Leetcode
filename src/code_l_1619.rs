struct Solution;

impl Solution {

    // https://leetcode.cn/problems/mean-of-array-after-removing-some-elements/
    #[allow(dead_code)]
    pub fn trim_mean(mut arr: Vec<i32>) -> f64 {
        let n = arr.len();
        let start = n / 20;
        let end = n - n / 20;
        arr.sort();
        arr[start..end].iter().sum::<i32>() as f64 / (n - n / 10) as f64
    }
}

#[cfg(test)]
mod test{
    use super::Solution;

    #[test]
    fn test() {
        let arr = vec![1,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,3];
        let res = Solution::trim_mean(arr);
        assert!((2.00000 - res).abs() < 0.00001);
    }

    #[test]
    fn test1() {
        let arr = vec![6,0,7,0,7,5,7,8,3,4,0,7,8,1,6,8,1,1,2,4,8,1,9,5,4,3,8,5,10,8,6,6,1,0,6,10,8,2,3,4];
        let res = Solution::trim_mean(arr);
        assert!((4.77778f64 - res).abs() < 0.00001);
    }
}