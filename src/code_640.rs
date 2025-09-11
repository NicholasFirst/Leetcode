
struct Solution;

impl Solution {
    
    /// https://leetcode.cn/problems/solve-the-equation/
    #[allow(dead_code)]
    pub fn solve_equation(equation: String) -> String {
        let equation = equation.replace("-", "+-");
        let mut ex = equation.split("=");
        let (a, b) = Self::parse(ex.next().unwrap());
        let (c, d) = Self::parse(ex.next().unwrap());
        if a == c {
            if b == d {
                "Infinite solutions".to_string()
            } else {
                "No solution".to_string()
            }
        } else {
            format!("x={}", (d - b) / (a - c))  // 常数项/x的项数 = 结果
        }
    }

    fn parse(expr: &str)  -> (i32, i32){
        let mut a = 0;
        let mut b = 0;
        for item in expr.split("+") {
            if item == "x" {
                a += 1;
            } else if item == "-x" {
                a -= 1;
            } else if item.ends_with("x") {     //处理X前面的常数项
                a += item[..item.len()-1].parse::<i32>().unwrap();
            } else if !item.is_empty() {
                b += item.parse::<i32>().unwrap();
            }
        }
        (a, b)
    }
}

#[cfg(test)]
mod test{
    use super::Solution;

    #[test]
    fn test() {
        let res = Solution::solve_equation("x+5-3+x=6+x-2".to_owned());
        assert_eq!("x=2".to_owned(), res);
    }

    #[test]
    fn test1() {
        let res = Solution::solve_equation("x=x".to_owned());
        assert_eq!("Infinite solutions".to_owned(), res);
    }

    #[test]
    fn test2() {
        let res = Solution::solve_equation("2x=x".to_owned());
        assert_eq!("x=0".to_owned(), res);
    }

    #[test]
    fn test3() {
        let res = Solution::solve_equation("0x=0".to_owned());
        assert_eq!("Infinite solutions".to_owned(), res);
    }

    /// 对角线遍历
    /// https://leetcode.cn/leetbook/read/array-and-string/cuxq3/
    #[allow(dead_code)]
    #[allow(unused)]
    pub fn find_diagonal_order(mat: Vec<Vec<i32>>) -> Vec<i32> {
        if mat.is_empty() || mat[0].is_empty() {
            return vec![];
        }
        let m = mat.len();
        let n = mat[0].len();
        let mut result = Vec::with_capacity(m * n);
        
        for s in 0..(m + n - 1) {
            if s % 2 == 0 {
                // Even sum: from bottom to top
                let i = std::cmp::min(s, m - 1);
                let mut j = s - i;
                for i in (0..=i).rev() {
                    if j >= n {
                        break;
                    }
                    result.push(mat[i][j]);
                    j += 1;
                }
            } else {
                // Odd sum: from top to bottom
                let j = std::cmp::min(s, n - 1);
                let mut i = s - j;
                for j in (0..=j).rev() {
                    if i >= m {
                        break;
                    }
                    result.push(mat[i][j]);
                    i += 1;
                }
            }
        }
        
        result
    }

    #[test]
    fn test_order() {
        let mat = vec![
            vec![1,2,3], 
            vec![4,5,6], 
            vec![7,8,9]
        ];
        let res = find_diagonal_order(mat);
        assert_eq!(vec![1,2,4,7,5,3,6,8,9], res);
    }
}