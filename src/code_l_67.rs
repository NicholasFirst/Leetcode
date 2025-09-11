struct Solution;

impl Solution {
 
    // https://leetcode.cn/problems/add-binary/description/
    #[allow(dead_code)]
    pub fn add_binary(a: String, b: String) -> String {
        let mut vec_a:Vec<i32> = a.chars().map(|c|c.to_digit(10).unwrap() as i32).collect();
        let mut vec_b:Vec<i32> = b.chars().map(|c|c.to_digit(10).unwrap() as i32).collect();
        let len = vec_a.len().max(vec_b.len());
        
        let mut r_vec: Vec<i32> = Vec::new();
        
        let mut carry:i32 = 0;   //进位标志
        
        for _ in 0..len { //挨个处理
            let l_a = vec_a.pop().unwrap_or(0);
            let l_b = vec_b.pop().unwrap_or(0);
            let mut r_s = l_a + l_b + carry; //每一位等于a中的位+b位+进位值
            carry = 0;  //加过后,进位值已经被消耗了
            if r_s >=2 {
                r_s -=2;  //相加的结果如果大于2, 就减掉2, 并将进位设置为1
                carry = 1;
            }
            r_vec.insert(0, r_s) //在结果vec中最前面插入结果和
        }

        if carry ==1 {  //循环完毕后,检查进位标志,如果是1, 就多插入一位
            r_vec.insert(0, 1) 
        }
        // 转为string
        let joined:String = r_vec.iter().map(ToString::to_string).collect();
        return joined
    }
}


#[cfg(test)]
mod test{
    use super::Solution;

    #[test]
    fn test() {
        let a = "1010".to_string();
        let b = "1011".to_string();
        let res = Solution::add_binary(a, b);
        assert_eq!("10101".to_string(), res);
    }
}