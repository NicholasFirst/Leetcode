struct Solution;

impl Solution{
    // https://leetcode.cn/problems/maximum-value-at-a-given-index-in-a-bounded-array/
    // DFS
    #[allow(dead_code)]
    pub fn max_value(n: i32, index: i32, max_sum: i32) -> i32 {
        let n = (n - 1) as usize;
        let index = index as usize;
        let max_sum = max_sum as usize;
        let mut len_l = 0;          //左侧扩展长度
        let mut len_r = 0;          //右侧扩展长度
        let mut total = n + 1;      //累加和(数组初始值均为1)
        let mut index_num = 1usize; //index位置值
        while max_sum >= total + len_l + len_r + 1 {
            index_num += 1;
            total += len_l + len_r + 1;
            if index - len_l > 0 {
                len_l += 1;
            }
            if index + len_r < n {
                len_r += 1;
            }
        }
        index_num as i32
    }
}

#[cfg(test)]
mod test{
    use super::Solution;

    #[test]
    fn test() {
        let n = 4;
        let index = 2;
        let max_sum = 6;
        let res = Solution::max_value(n, index, max_sum);
        assert_eq!(2, res);
    }


    #[test]
    fn test1() {
        let n = 6;
        let index = 1;
        let max_sum = 10;
        let res = Solution::max_value(n, index, max_sum);
        assert_eq!(3, res);
    }

    #[test]
    fn test2() {
        let n = 3;
        let index = 2;
        let max_sum = 18;
        let res = Solution::max_value(n, index, max_sum);
        assert_eq!(7, res);
    }

    #[test]
    fn test3() {
        let n = 4;
        let index = 0;
        let max_sum = 4;
        let res = Solution::max_value(n, index, max_sum);
        assert_eq!(1, res);
    }

    #[test]
    fn test4() {
        let n = 2;
        let index = 0;
        let max_sum = 798870804;
        let res = Solution::max_value(n, index, max_sum);
        assert_eq!(399435402, res);
    }
}