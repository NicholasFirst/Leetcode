use std::collections::VecDeque;

struct Solution;

impl Solution {
    //https://leetcode.cn/problems/find-the-array-concatenation-value/
    ///给你一个下标从 0 开始的整数数组 nums 。
    ///现定义两个数字的 串联 是由这两个数值串联起来形成的新数字。
    ///例如，15 和 49 的串联是 1549 。
    ///nums 的 串联值 最初等于 0 。执行下述操作直到 nums 变为空：
    ///如果 nums 中存在不止一个数字，分别选中 nums 中的第一个元素和最后一个元素，将二者串联得到的值加到 nums 的 串联值 上，然后从 nums 中删除第一个和最后一个元素。
    ///如果仅存在一个元素，则将该元素的值加到 nums 的串联值上，然后删除这个元素。
    ///返回执行完所有操作后 nums 的串联值。
    ///
    #[allow(dead_code)]
    pub fn find_the_array_conc_val(nums: Vec<i32>) -> i64 {
        let n = nums.len();
        let mut ans = 0;
        let mut i = 0;
        let mut j = n - 1;
        while i < j {
            ans += format!("{}{}", nums[i], nums[j]).parse::<i64>().unwrap();
            i += 1;
            j -= 1;
        }
        if i == j {
            ans += nums[i] as i64;
        }
        ans
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test() {
        let nums = vec![7, 52, 2, 4];
        let res = Solution::find_the_array_conc_val(nums);
        assert_eq!(596, res);
        let nums = vec![5, 14, 13, 8, 12];
        let res = Solution::find_the_array_conc_val(nums);
        assert_eq!(673, res);
    }
}
