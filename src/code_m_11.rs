struct Solution;

impl Solution {
    
    /// https://leetcode.cn/problems/container-with-most-water/
    /// 双指针
    #[allow(dead_code)]
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut res = 0;
        let (mut l, mut r) = (0, height.len() - 1);
        while l < r {
            res = res.max(height[l].min(height[r]) * (r - l) as i32);
            if height[l] < height[r] {
                l += 1;
            } else {
                r -= 1;
            }
        }
        res
    }
}

#[cfg(test)]
mod test{
    use super::Solution;

    #[test]
    fn test() {
        let height = vec![1,8,6,2,5,4,8,3,7];
        let res = Solution::max_area(height);
        assert_eq!(res, 49);
    }
}