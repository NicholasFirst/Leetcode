struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn distinct_integers(n: i32) -> i32 {
        let n = n as usize;
        let mut nums = vec![0; n + 1];
        nums[n] = 1;
        for _k in 0..n {
            for x in 1..=n {
                if nums[x] == 0 {
                    continue;
                }
                for i in 1..=n {
                    if x % i == 1 {
                        nums[i] = 1;
                    }
                }
            }
        }
        nums.iter().sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let n = 5;
        let result = Solution::distinct_integers(n);
        assert_eq!(result, 4);
    }

    fn test2() {
        let n = 3;
        let result = Solution::distinct_integers(n);
        assert_eq!(result, 2);
    }
}