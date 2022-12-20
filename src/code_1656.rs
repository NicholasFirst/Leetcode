/// https://leetcode.cn/problems/design-an-ordered-stream/
#[derive(Debug)]
struct OrderedStream {
    pre: usize,
    data: Vec<Option<String>>,
}

impl OrderedStream {

    #[allow(dead_code)]
    fn new(n: i32) -> Self {
        OrderedStream{
            pre: 0,
            data: vec![None; n as usize]
        }
    }
    
    #[allow(dead_code)]
    fn insert(&mut self, id_key: i32, value: String) -> Vec<String> {
        let index =  (id_key - 1) as usize;
        let _ = std::mem::replace(&mut self.data[index], Some(value));
        if index == self.pre {
            let mut ans: Vec<String> = Vec::new();
            for item in self.data.iter().skip(index) {
                if let Some(s) = item {
                    ans.push(s.clone());
                    self.pre += 1;
                }else{
                    break;
                }
            }
            return ans;
        }
        return vec![]
    }
}

#[cfg(test)]
mod test {
    use super::OrderedStream;

    #[test]
    fn test() {
        let mut obj = OrderedStream::new(5);
        let res1 = obj.insert(3, "ccccc".to_string());
        let res2 = obj.insert(1, "aaaaa".to_string());
        let res3 = obj.insert(2, "bbbbb".to_string());
        let res4 = obj.insert(5, "eeeee".to_string());
        let res5 = obj.insert(4, "ddddd".to_string());
        let empty: Vec<String> = vec![];
        assert_eq!(empty, res1);
        assert_eq!(vec!["aaaaa"], res2);
        assert_eq!(vec!["bbbbb", "ccccc"], res3);
        assert_eq!(empty, res4);
        assert_eq!(vec!["ddddd", "eeeee"], res5);
        println!("{:?}", obj);
    }
}