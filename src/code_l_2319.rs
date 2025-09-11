struct Solution;


impl Solution {

    // https://leetcode.cn/problems/check-if-matrix-is-x-matrix/
    #[allow(dead_code)]
    pub fn check_x_matrix(grid: Vec<Vec<i32>>) -> bool {
        grid.into_iter().enumerate().all(|(n,x)|x.iter().enumerate().all(|(m,&y)|(y==0)^(m==n || m+n==x.len()-1)))
    }
}

#[cfg(test)]
mod test{
    use super::Solution;

    #[test]
    fn test() {
        let grid = vec![vec![2,0,0,1],vec![0,3,1,0],vec![0,5,2,0],vec![4,0,0,2]];
        let ans = Solution::check_x_matrix(grid);
        assert!(ans);
    }
}