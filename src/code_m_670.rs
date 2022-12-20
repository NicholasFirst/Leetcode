struct Solution;

impl Solution{
    // https://leetcode.cn/problems/maximum-swap/
    // 贪心算法..
    #[allow(dead_code)]
    pub fn maximum_swap(num: i32) -> i32 {
        let mut s: Vec<_> = format!("{}", num).to_string().bytes().collect();
        let (mut max, mut max_id) = (b'0' - 1, s.len());
        let (mut id1, mut id2) = (s.len(), s.len());
        for i in (0..s.len()).rev() {
            if s[i]  > max {
                max = s[i];
                max_id = i;
            } else if s[i] < max {
                id1 = max_id;
                id2 = i;
            }
        }

        if id1 != s.len() && id2 != s.len() {
            s.swap(id1, id2);
            String::from_utf8(s).unwrap().parse().unwrap()
        } else {
            num
        }
    }
}


#[cfg(test)]
mod test{
    use super::Solution;

    #[test]
    fn test() {
        let num = 2736;
        let res = Solution::maximum_swap(num);
        assert_eq!(res, 7236);
    }

    #[test]
    fn test1() {
        let num = 9973;
        let res = Solution::maximum_swap(num);
        assert_eq!(res, 9973);
    }
}