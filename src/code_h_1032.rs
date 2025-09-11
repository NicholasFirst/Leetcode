struct StreamChecker {
    words: Vec<String>,
    s: String,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl StreamChecker {
    #[allow(dead_code)]
    fn new(words: Vec<String>) -> Self {
        StreamChecker {
            words,
            s: "".to_string(),
        }
    }

    #[allow(dead_code)]
    fn query(&mut self, letter: char) -> bool {
        self.s.push(letter);
        for word in &self.words {
            if self.s.ends_with(word) {
                return true;
            }
        }
        return false;
    }
}

#[cfg(test)]
mod test {
    use super::StreamChecker;

    #[test]
    fn test() {
        let words = vec!["cd".to_string(), "f".to_string(), "kl".to_string()];
        let mut obj = StreamChecker::new(words);
        let query_char = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l'];
        let output: Vec<bool> = query_char.into_iter().map(|c| obj.query(c)).collect();
        assert_eq!(
            vec![false, false, false, true, false, true, false, false, false, false, false, true],
            output
        );
    }
}
