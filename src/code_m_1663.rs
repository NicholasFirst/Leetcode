struct Solution;

impl Solution {

    // https://leetcode.cn/problems/smallest-string-with-a-given-numeric-value/
    #[allow(dead_code)]
    pub fn get_smallest_string(n: i32, k: i32) -> String {
        let l = k - n;
        let mut res_arr = vec![97_u8; n as usize];
        let last = (l % 25) as u8;
        let t = (l / 25) as usize;
        let iter = res_arr.iter_mut();
        iter.take(t + 1).map(|x| {
            *x += 25;
            x
        }).skip(t).map(|y| {
            *y -= 25;
            *y += last;
        }).count();
        res_arr.reverse();
        String::from_utf8(res_arr).expect("error")
    }
}


#[cfg(test)]
mod test{
    use super::Solution;

    #[test]
    fn test() {
        let n = 5;
        let k = 73;
        let res = Solution::get_smallest_string(n, k);
        assert_eq!("aaszz".to_string(), res);
    }


    #[test]
    fn test1() {
        let n = 3;
        let k = 27;
        let res = Solution::get_smallest_string(n, k);
        assert_eq!("aay".to_string(), res);
    }
}