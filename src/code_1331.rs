#[allow(dead_code)]
struct Solution;

impl Solution{
    
    /// 来自leetcode: https://leetcode.cn/problems/rank-transform-of-an-array/
    #[allow(dead_code)]    
    pub fn array_rank_transform(arr: Vec<i32>) -> Vec<i32> {
        if arr.is_empty() {
            return vec![];
        }
        let mut a = arr
            .iter()
            .enumerate()
            .map(|(i, n)| (n, i))
            .collect::<Vec<_>>();
        a.sort_unstable();
        let mut ans = vec![0; a.len()];
        ans[a[0].1] = 1;
        a.windows(2).for_each(|x| {
            print!("{:?}   ", x);
            ans[x[1].1] = ans[x[0].1] + if x[0].0 == x[1].0 {0} else {1};
            println!("{:?}", ans);
        });
        ans
    }
}


#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn test() {
        let arr = vec![40, 30, 50, 10];
        let res = Solution::array_rank_transform(arr);
        assert_eq!(vec![3, 2, 4, 1], res);
    }

    #[test]
    fn test1() {
        let arr = vec![10];
        let res = Solution::array_rank_transform(arr);
        assert!(vec![1] == res);
    }


    #[test]
    fn test3() {
        let mut a = vec![(40i32, 0usize),(30i32, 1usize),(50i32, 2usize),(10i32, 3usize)];
        a.sort_unstable();
        println!("{:?}", a); 
    }
}