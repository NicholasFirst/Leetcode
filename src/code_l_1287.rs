struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn find_special_integer(arr: Vec<i32>) -> i32 {
        let n = arr.len();
        let span = (n as f64 / 4.0).floor() as usize + 1;
        for i in (0..n).step_by(span) {
            let start = Self::binary_search(&arr, arr[i]);
            let end = Self::binary_search(&arr, arr[i] + 1);
            if end - start >= span as i32 {
                return arr[i];
            }
        }
        -1
    }

    fn binary_search(arr: &Vec<i32>, target: i32) -> i32 {
        let mut lo = 0 as i32;
        let mut hi = arr.len() as i32 - 1;
        let mut res = arr.len() as i32;
        while lo <= hi {
            let mid = (lo + hi) / 2;
            if arr[mid as usize] >= target {
                res = mid;
                hi = mid - 1;
            } else {
                lo = mid + 1;
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_1287() {
        assert_eq!(
            Solution::find_special_integer(vec![1, 2, 2, 6, 6, 6, 6, 7, 10]),
            6
        );
        assert_eq!(
            Solution::find_special_integer(vec![
                1, 1, 2, 2, 2, 2, 3, 3, 3, 3, 4, 4, 4, 4, 5, 5, 5, 5
            ]),
            -1
        );
        assert_eq!(
            Solution::find_special_integer(vec![2, 2, 2, 2, 2, 2, 2, 2, 2, 2]),
            2
        );
    }
}
