struct TextEditor {
    ptr: usize,
    text: Vec<u8>,
}

/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
 impl TextEditor {
    fn new() -> Self {
        TextEditor {
            ptr: 0,
            text: vec![],
        }
    }
    
    fn add_text(&mut self, text: String) {
        let text_bytes: Vec<u8> = text.as_bytes().into();
        self.text.splice(self.ptr..self.ptr, text_bytes);
        self.ptr += text.len();
    }
    
    fn delete_text(&mut self, k: i32) -> i32 {
        let k = (k as usize).min(self.ptr);
        self.text.drain((self.ptr - k)..self.ptr);
        self.ptr -= k;
        k as i32
    }
    
    fn cursor_left(&mut self, k: i32) -> String {
        self.ptr = (self.ptr as i32 - k).max(0) as usize;
        String::from_utf8_lossy(&self.text[self.ptr - 10.min(self.ptr)..self.ptr]).to_string()
    }
    
    fn cursor_right(&mut self, k: i32) -> String {
        self.ptr = (self.ptr as i32 + k).min(self.text.len() as i32) as usize;
        String::from_utf8_lossy(&self.text[self.ptr - 10.min(self.ptr)..self.ptr]).to_string()
    }
}

/*
 * Your TextEditor object will be instantiated and called as such:
 * let obj = TextEditor::new();
 * obj.add_text(text);
 * let ret_2: i32 = obj.delete_text(k);
 * let ret_3: String = obj.cursor_left(k);
 * let ret_4: String = obj.cursor_right(k);
 */

 #[cfg(test)]
 mod tests {

     use super::*;

     #[test]
     fn test_1() {
         let mut obj = TextEditor::new();
         obj.add_text("leetcode".to_string());
         assert_eq!(obj.delete_text(4), 4);
         obj.add_text("practice".to_string());
         assert_eq!(obj.cursor_left(3), "leetpract".to_string());
         assert_eq!(obj.cursor_right(8), "etpractice".to_string());
     }
 }