struct Solution;

impl Solution {
    // https://leetcode.cn/problems/rearrange-spaces-between-words/
    #[allow(dead_code)]
    pub fn reorder_spaces(text: String) -> String {
        let mut whitespaces = text.chars().filter(|&ch| ch == ' ').count();
        let words = text.split_ascii_whitespace().collect::<Vec<&str>>();
        let mut res = String::new();
        let span = if words.len() > 1 { whitespaces / (words.len() - 1) } else { whitespaces + 1 };
        for word in words {
            res.push_str(word);
            if whitespaces >= span {
                res.push_str(&" ".repeat(span));
                whitespaces -= span;
            }
        }
        if whitespaces > 0 { res.push_str(&" ".repeat(whitespaces)); }
        res
    }
}


#[cfg(test)]
mod test{
    use super::Solution;

    #[test]
    fn test() {
        let text = "  this   is  a sentence ".to_string();
        let res = Solution::reorder_spaces(text);
        assert_eq!("this   is   a   sentence".to_string(), res);
    }

    #[test]
    fn test1() {
        let text = " practice   makes   perfect".to_string();
        let res = Solution::reorder_spaces(text);
        assert_eq!("practice   makes   perfect ".to_string(), res);
    }

    #[test]
    fn test2() {
        let text = "  walks  udp package   into  bar a".to_string();
        let res = Solution::reorder_spaces(text);
        assert_eq!("walks  udp  package  into  bar  a ".to_string(), res);
    }

    #[test]
    fn test3() {
        let text = "a".to_string();
        let res = Solution::reorder_spaces(text);
        assert_eq!("a".to_string(), res);
    }
}