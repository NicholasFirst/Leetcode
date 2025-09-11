struct Solution;

impl Solution {
    // https://leetcode.cn/problems/masking-personal-information/
    #[allow(dead_code)]
    pub fn mask_pii(s: String) -> String {
        if s.contains('@') {
            let mut index = 0;
            let mut email = s.chars().enumerate().map(|(i, c)| {
                if c == '@' {
                    index = i;
                }
                c.to_ascii_lowercase()
            }).collect::<String>();
            if index == 2 {
                email.insert_str(1, "*****");
                return email;
            } else {
                let replace: String = email.chars().skip(1).take(index - 2).collect::<String>();
                return email.replacen(replace.as_str(), "*****", 1);
            }
        } else {
            const COUNTRY: [&'static str; 4] = ["", "+*-", "+**-", "+***-"];
            let s = s
                .replace("+", "")
                .replace("-", "")
                .replace("(", "")
                .replace(")", "")
                .replace(" ", "");
            format!("{}***-***-{}", COUNTRY[s.len() - 10], &s[s.len() - 4..])
        }
    }
}

#[cfg(test)]
mod test{
    use super::Solution;

    #[test]
    fn test1() {
        let s = "LeetCode@LeetCode.com".to_string();
        let res = Solution::mask_pii(s);
        assert_eq!("l*****e@leetcode.com".to_string(), res);
    }


    #[test]
    fn test2() {
        let s = "AB@qq.com".to_string();
        let res = Solution::mask_pii(s);
        assert_eq!("a*****b@qq.com".to_string(), res);
    }


    #[test]
    fn test3() {
        let s = "86-(10)12345678".to_string();
        let res = Solution::mask_pii(s);
        assert_eq!("+**-***-***-5678".to_string(), res);
    }
}