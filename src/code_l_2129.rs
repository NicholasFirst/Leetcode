struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn capitalize_title(title: String) -> String {
        let title = title.to_lowercase();

        title
            .split_whitespace()
            .map(|s| {
                if s.len() > 2 {
                    s.chars()
                        .nth(0)
                        .unwrap()
                        .to_uppercase()
                        .chain(s.chars().skip(1))
                        .collect::<String>()
                } else {
                    s.to_owned()
                }
            })
            .collect::<Vec<String>>()
            .join(" ")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_capitalize_title() {
        let title = "capiTalIze tHe titLe";
        Solution::capitalize_title(title.to_string());
        assert_eq!(
            Solution::capitalize_title(title.to_string()),
            "Capitalize The Title"
        );

        let title = "First leTTeR of EACH Word";
        Solution::capitalize_title(title.to_string());
        assert_eq!(
            Solution::capitalize_title(title.to_string()),
            "First Letter of Each Word"
        );

        let title = "i lOve leetcode";
        Solution::capitalize_title(title.to_string());
        assert_eq!(
            Solution::capitalize_title(title.to_string()),
            "i Love Leetcode"
        );
    }
}
