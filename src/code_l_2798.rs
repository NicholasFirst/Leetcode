struct Solution;


impl Solution {
    #[allow(dead_code)]
    // https://leetcode.cn/problems/number-of-employees-who-met-the-target/?envType=daily-question&envId=2024-05-04
    pub fn number_of_employees_who_met_target(hours: Vec<i32>, target: i32) -> i32 {
        hours.iter().filter(|&h|  h >= &target).count() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let hours = vec![0,1,2,3,4];
        let target = 2;
        let emps = Solution::number_of_employees_who_met_target(hours, target);
        assert_eq!(emps, 3);
    }
}