struct Solution;

impl Solution {
    #[allow(dead_code)]
    // https://leetcode.cn/problems/min-cost-climbing-stairs/?envType=daily-question&envId=2023-12-17
    pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
        // https://leetcode.cn/problems/min-cost-climbing-stairs/?envType=daily-question&envId=2023-12-17
        // 对方题解
        fn dfs(i: usize, memo: &mut Vec<i32>, cost: &Vec<i32>) -> i32 {
            if i <= 1 { // 递归边界
                return 0;
            }
            if memo[i] != -1 { // 之前计算过
                return memo[i];
            }
            let res1 = dfs(i - 1, memo, cost) + cost[i - 1];
            let res2 = dfs(i - 2, memo, cost) + cost[i - 2];
            let res = res1.min(res2);
            memo[i] = res; // 记忆化
            res
        }
        let n = cost.len();
        let mut memo = vec![-1; n + 1]; // -1 表示没有计算过
        dfs(n, &mut memo, &cost)
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_1() {
        // let input = [10, 15, 20];
        let input = vec![1, 100, 1, 1, 1, 100, 1, 1, 100, 1];
        let output = 6;
        assert_eq!(Solution::min_cost_climbing_stairs(input), output);
    }

    #[test]
    fn test_2() {
        let input = vec![10,15,20];
        let output = 15;
        assert_eq!(Solution::min_cost_climbing_stairs(input), output);
    }
}