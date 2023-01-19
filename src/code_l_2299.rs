struct Solution;

impl Solution {

    // https://leetcode.cn/problems/strong-password-checker-ii/
    #[allow(dead_code)]
    pub fn strong_password_checker_ii(password: String) -> bool {
        let tar: Vec<u8> = "!@#$%^&*()-+".as_bytes().into();
        let byt = password.as_bytes();
        if byt.len() < 7 {
            return false;
        }
        let mut one = false;
        let mut two = false;
        let mut three = false;
        let mut four = false;
        let mut five = true;
        let mut c = u8::MAX;
        byt.iter().for_each(|&x| {
            if !one {
                if 122 >= x && 97 <= x {
                    one = true;
                }
            }
            if !two {
                if 90 >= x && 65 <= x {
                    two = true;
                }
            }
            if !three {
                if 57 >= x && 48 <= x {
                    three = true;
                }    
            }
            if !four {
                if tar.contains(&x) {
                    four = true;
                }
            }
            if five {
                if c == x {
                    five = false;
                }
                c = x;
            }
        });
        one && two && three && four && five
    }
}

#[cfg(test)]
mod test{
    use super::Solution;

    #[test]
    fn test() {
        let password = "IloveLe3tcode!".to_string();
        let bol = Solution::strong_password_checker_ii(password);
        assert!(bol);
    }

    #[test]
    fn test1() {
        let password = "Me+You--IsMyDream".to_string();
        let bol = Solution::strong_password_checker_ii(password);
        assert!(!bol);
    }
}