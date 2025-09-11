struct Solution;

impl Solution {
    // https://leetcode.cn/problems/sort-vowels-in-a-string/description/?envType=daily-question&envId=2025-09-11
    #[allow(dead_code)]
    pub fn sort_vowels(s: String) -> String {
        let mut s = s.chars().collect::<Vec<char>>();
        let mut vowels = vec![];
        for i in 0..s.len() {
            if Solution::is_vowel(s[i]) {
                vowels.push(s[i]);
                s[i] = ' ';
            }
        }
        vowels.sort();
        let mut j = 0;
        for i in 0..s.len() {
            if s[i] == ' ' {
                s[i] = vowels[j];
                j += 1;
            }
        }
        s.into_iter().collect::<String>()
    }

    fn is_vowel(c: char) -> bool {
        c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u' || c == 'A' || c == 'E' || c == 'I' || c == 'O' || c == 'U'
    }
}


#[cfg(test)]
mod tests {
    
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::sort_vowels("lEetcOde".to_string()), "lEOtcede".to_string());
        assert_eq!(Solution::sort_vowels("lYmpH".to_string()), "lYmpH".to_string());
    }
}