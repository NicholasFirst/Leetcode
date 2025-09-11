struct Solution;

impl Solution {

    // https://leetcode.cn/problems/count-special-quadruplets/description/
    #[allow(dead_code)]
    pub fn count_quadruplets(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut ans = 0;
        for a in 0..n {
            for b in a + 1..n {
                for c in b + 1 .. n {
                    for d in c + 1 .. n {
                        if nums[a] + nums[b] + nums[c] == nums[d] {
                            ans += 1;
                        }
                    }
                }
            }
        }
        ans
    }
}


#[cfg(test)]
mod test{
    use super::Solution;

    #[test]
    fn test() {
        let nums = vec![1,2,3,6];
        let ans = Solution::count_quadruplets(nums);
        assert_eq!(1, ans);
    }

    #[test]
    fn test1() {
        let nums = vec![1,1,1,3,5];
        let ans = Solution::count_quadruplets(nums);
        assert_eq!(4, ans);
    }
}