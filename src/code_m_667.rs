struct Solution;

impl Solution {
    // https://leetcode.cn/problems/beautiful-arrangement-ii/
    #[allow(dead_code)]
    pub fn construct_array(n: i32, mut k: i32) -> Vec<i32> {
        let (mut l, mut r, mut res) = (1, n, Vec::with_capacity(n as usize));
        for _ in 0..n {
            if k % 2 == 0 {
                res.push(r);
                r -= 1;
            } else {
                res.push(l);
                l += 1;
            };
            if k > 1 { k -= 1; }
        }
        res
    }
}

#[cfg(test)]
mod test{
    use super::Solution;

    #[test]
    fn test() {
        let res = Solution::construct_array(3,1);
        assert_eq!(res, vec![1,2,3]);
    }
}