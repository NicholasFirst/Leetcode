#[allow(dead_code)]
struct Solution;

impl Solution {
    /// https://leetcode-cn.com/problems/find-the-middle-index-in-array/
    #[allow(dead_code)]
    pub fn find_middle_index(nums: Vec<i32>) -> i32 {
        let total = nums.iter().sum::<i32>();
        let mut sub = 0;
        for (idx, num) in nums.iter().enumerate() {
            if (total - num) % 2 == 0 {
                let half = (total - num) / 2;
                if sub == half {
                    return idx as i32;
                }
            }
            sub += num;       
        }
        -1
    }
}


#[cfg(test)]
mod test{
    use super::Solution;

    #[test]
    fn test() {
        let nums = vec![1,1];
        let res = Solution::find_middle_index(nums);
        assert_eq!(-1, res);
    }

    #[test]
    fn test1() {
        let nums = vec![1, 7, 3, 6, 5, 6];
        let res = Solution::find_middle_index(nums);
        assert_eq!(3, res);
    }

    #[test]
    fn test2() {
        let nums = vec![1, 2, 3];
        let res = Solution::find_middle_index(nums);
        assert_eq!(-1, res);
    }

    #[test]
    fn test3() {
        let nums = vec![2,3,-1,8,4];
        let res = Solution::find_middle_index(nums);
        assert_eq!(3, res);
    }

    #[test]
    fn test4() {
        let nums = vec![0,0,0,0,0,0,1];
        let res = Solution::find_middle_index(nums);
        assert_eq!(6, res);
    }
}