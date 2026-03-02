
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn max_chunks_to_sorted(arr: Vec<i32>) -> i32 {
        let mut ans = 1;
        if arr.len() <= 1 {
            return 1;
        }
        arr.windows(2).for_each(|x| {
            if x[0] < x[1] {
                ans += 1;
            }
        });
        ans
    }
}


#[cfg(test)]
mod test{
    use super::Solution;

    #[test]
    fn test() {
        let arr = vec![1,0,2,3,4];
        let ans = Solution::max_chunks_to_sorted(arr);
        assert_eq!(ans, 4);
    }

    #[test]
    fn test1() {
        let arr = vec![4,3,2,1,0];
        let ans = Solution::max_chunks_to_sorted(arr);
        assert_eq!(ans, 1);
    }

    #[test]
    fn test2() {
        let arr = vec![2,0,1];
        let ans = Solution::max_chunks_to_sorted(arr);
        assert_eq!(ans, 2);
    }
}