struct Solution;


impl Solution {

    #[allow(dead_code)]
    // https://leetcode.cn/problems/count-tested-devices-after-test-operations/?envType=daily-question&envId=2024-05-12
    pub fn count_tested_devices(battery_percentages: Vec<i32>) -> i32 {
        let mut res = 0;
        let mut used = 0;
        let len = battery_percentages.len();
        for i in 0..len {
            if i == len - 1 && battery_percentages[i] - used > 0{
                res += 1;
                break;
            }
            if battery_percentages[i] - used > 0 {
                res += 1;
                used += 1;

            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        let battery_percentages = vec![1,1,2,1,3];
        let res = Solution::count_tested_devices(battery_percentages);
        assert_eq!(res, 3);
    }

    #[test]
    fn test_solution2() {
        let battery_percentages = vec![0,1,2];
        let res = Solution::count_tested_devices(battery_percentages);
        assert_eq!(res, 2);
    }
}