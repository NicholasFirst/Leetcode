struct Solution;

impl Solution {

    // https://leetcode.cn/problems/excel-sheet-column-title/
    #[allow(dead_code)]
    pub fn convert_to_title(column_number: i32) -> String {
        let mut ans = String::new();
        let mut num = column_number;
        while num > 0 {
            let last = (num - 1) % 26;
            let c = ('A' as u8) + (last as u8);
            ans.push(c as char);
            num = (num - 1) / 26;
        }
        return ans.chars().rev().collect();
    }
}

#[cfg(test)]
mod test{
    use super::Solution;

    #[test]
    fn test() {
        let column_number = 52;
        let res = Solution::convert_to_title(column_number);
        assert_eq!(res, "AZ".to_string());
    }


    #[test]
    fn test1() {
        let column_number = 701;
        let res = Solution::convert_to_title(column_number);
        assert_eq!(res, "ZY".to_string());
    }

    #[test]
    fn test2() {
        let column_number = 2147483647;
        let res = Solution::convert_to_title(column_number);
        assert_eq!(res, "FXSHRXW".to_string());
    }
}