struct Solution;

impl Solution{
    #[allow(dead_code)]
    pub fn frequency_sort(nums: Vec<i32>) -> Vec<i32> {
        let mut n = vec![0; 201];
        nums.iter().for_each(|&x| {
            n[(x + 100) as usize] += 1;
        });
        let mut f = vec![];
        for (i, &n) in n.iter().enumerate() {
            if n > 0 {
                f.push((n as usize, i as i32 - 100));
            }
        }
        f.sort_unstable_by(|a, b| a.0.cmp(&b.0).then(b.1.cmp(&a.1)));
        let mut res = vec![];
        for (n, x) in f {
            res = [res, vec![x; n]].concat();
        }
        res
    }
}

#[cfg(test)]
mod test{
    use super::Solution;

    #[test]
    fn test() {
        let nums = vec![1,1,2,2,2,3];
        let res = Solution::frequency_sort(nums);
        assert_eq!(vec![3,1,1,2,2,2], res);
    }
}