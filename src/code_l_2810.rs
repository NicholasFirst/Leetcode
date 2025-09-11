struct Solution;

impl Solution {
    #[allow(dead_code)]
    // https://leetcode.cn/problems/faulty-keyboard/?envType=daily-question&envId=2024-05-05
    pub fn final_string(s: String) -> String {
        let mut stack = vec![];
        s.chars().for_each(|c| {
            if c == 'i' {
                stack.reverse();
            }else {
                stack.push(c);
            }
        });
        stack.into_iter().collect()
        // let mut stack = vec![];
        // let mut head = false;
        // for ch in s.chars() {
        //     if ch != 'i' {
        //         if head {
        //             stack.insert(0, ch);
        //         } else {
        //             stack.push(ch);
        //         }
        //     } else {
        //         head = !head;
        //     }
        // }
        // let res = if head {
        //     stack.iter().rev().collect()
        // } else {
        //     stack.iter().collect()
        // };
        // res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let s = "string".to_string();
        let result = Solution::final_string(s);
        assert_eq!(result, "rtsng".to_string());
    }

    #[test]
    fn test2() {
        let s = "poiinter".to_string();
        let result = Solution::final_string(s);
        assert_eq!(result, "ponter".to_string());
    }
}
