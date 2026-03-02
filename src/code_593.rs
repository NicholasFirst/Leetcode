
#[allow(dead_code)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn valid_square(p1: Vec<i32>, p2: Vec<i32>, p3: Vec<i32>, p4: Vec<i32>) -> bool {
        p1 != p2 && p2 != p3 && p3 != p4 && p1 != p3 && p2 != p4 && p1 != p4
        && (p1[0] - p2[0]).pow(2) + (p1[1] - p2[1]).pow(2) == (p3[0] - p4[0]).pow(2) + (p3[1] - p4[1]).pow(2)
        && (p1[0] - p3[0]).pow(2) + (p1[1] - p3[1]).pow(2) == (p2[0] - p4[0]).pow(2) + (p2[1] - p4[1]).pow(2)
        && (p1[0] - p4[0]).pow(2) + (p1[1] - p4[1]).pow(2) == (p2[0] - p3[0]).pow(2) + (p2[1] - p3[1]).pow(2)
        && (
            (p1[0] - p4[0]).pow(2) + (p1[1] - p4[1]).pow(2) == (p1[0] - p3[0]).pow(2) + (p1[1] - p3[1]).pow(2)
            ||
            (p1[0] - p4[0]).pow(2) + (p1[1] - p4[1]).pow(2) == (p1[0] - p2[0]).pow(2) + (p1[1] - p2[1]).pow(2)
            ||
            (p1[0] - p3[0]).pow(2) + (p1[1] - p3[1]).pow(2) == (p1[0] - p2[0]).pow(2) + (p1[1] - p2[1]).pow(2)
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let p1 = vec![0,0];
        let p2 = vec![0,0];
        let p3 = vec![0,0];
        let p4 = vec![0,0];
        assert!(!Solution::valid_square(p1, p2, p3, p4));
    }

    #[test]
    fn test1() {
        let x = vec![0, 0];
        let y = vec![0, 0];
        assert!(x == y);
    }
}