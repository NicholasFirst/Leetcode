struct Solution;

impl Solution {

    #[allow(dead_code)]
    //https://leetcode.cn/problems/number-of-boomerangs/?envType=daily-question&envId=2024-01-08
    pub fn number_of_boomerangs(points: Vec<Vec<i32>>) -> i32 {
        todo!()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_number_of_boomerangs() {
        let result = Solution::number_of_boomerangs(vec![vec![0,0],vec![1,0],vec![2,0]]);
        assert_eq!(result, 2);
    }
}