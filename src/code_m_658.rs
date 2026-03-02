struct Solution;

impl Solution {
    /// https://leetcode.cn/problems/find-k-closest-elements/
    /// 下面使用的方法是DFS
    #[allow(dead_code)]
    pub fn find_closest_elements(arr: Vec<i32>, k: i32, x: i32) -> Vec<i32> {
        let mut total = i32::MAX;
        let mut index = 0usize;
        arr.windows(k as usize)
            .enumerate()
            .map(|(i, v)| {
                let sum: i32 = v.iter().map(|y| (y - x).abs()).sum();
                if sum < total {
                    total = sum;
                    index = i;
                }
            })
            .count();
        arr[index..index + (k as usize)].to_owned()
    }

    /// 双指针对撞法, 有空尝试实现
    /// 整数 a 比整数 b 更接近 x 需要满足：
    /// |a - x| < |b - x| 或者
    /// |a - x| == |b - x| 且 a < b
    #[allow(dead_code)]
    pub fn find_closest_elements_two(arr: Vec<i32>, k: i32, x: i32) -> Vec<i32> {
        if arr.len() == k as usize {
            return arr;
        }
        let mut s = 0usize;
        let mut e = arr.len() - 1 as usize;
        while (e - s) >= k as usize {
            if (arr[e] - x).abs() < (arr[s] - x).abs() {
                s += 1;
            }else{
                e -= 1;
            }
        }
        arr[s..=e].to_owned()
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test() {
        let arr = vec![1, 2, 3, 4, 5];
        let k = 4;
        let x = 3;
        let res = Solution::find_closest_elements(arr, k, x);
        assert_eq!(vec![1, 2, 3, 4], res);
    }

    #[test]
    fn test1() {
        let arr = vec![1, 1, 1, 10, 10, 10];
        let k = 1;
        let x = 9;
        let res = Solution::find_closest_elements(arr, k, x);
        assert_eq!(vec![10], res);
    }

    #[test]
    fn test_two() {
        let arr = vec![1, 2, 3, 4, 5];
        let k = 4;
        let x = 3;
        let res = Solution::find_closest_elements_two(arr, k, x);
        assert_eq!(vec![1, 2, 3, 4], res);
    }

    #[test]
    fn test1_two() {
        let arr = vec![1, 1, 1, 10, 10, 10];
        let k = 1;
        let x = 9;
        let res = Solution::find_closest_elements_two(arr, k, x);
        assert_eq!(vec![10], res);
    }

    #[test]
    fn test2_two() {
        let arr = vec![1];
        let k = 1;
        let x = 1;
        let res = Solution::find_closest_elements_two(arr, k, x);
        assert_eq!(vec![1], res);
    }

    #[test]
    fn test3_two() {
        let arr = vec![-2,-1,1,2,3,4,5];
        let k = 7;
        let x = 3;
        let res = Solution::find_closest_elements_two(arr, k, x);
        assert_eq!(vec![-2,-1,1,2,3,4,5], res);
    }


    #[test]
    fn test4_two() {
        let arr = vec![0,0,0,1,3,5,6,7,8,8];
        let k = 2;
        let x = 2;
        let res = Solution::find_closest_elements_two(arr, k, x);
        assert_eq!(vec![1,3], res);
    }
}
