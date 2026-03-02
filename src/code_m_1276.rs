struct Solution;

impl Solution {
    #[allow(dead_code)]
    // https://leetcode.cn/problems/number-of-burgers-with-no-waste-of-ingredients/?envType=daily-question&envId=2023-12-25
    pub fn num_of_burgers(tomato_slices: i32, cheese_slices: i32) -> Vec<i32> {
        // 数学法
        let mac2 = tomato_slices - (cheese_slices << 1);
        let tiny2 = (cheese_slices << 2) - tomato_slices;
        if mac2 < 0 || tiny2 < 0 || mac2 & 1 != 0 || tiny2 & 1 != 0 {
            vec![]
        } else {
            vec![mac2 >> 1, tiny2 >> 1]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_num_of_burgers() {
        let tomato_slices = 16; 
        let cheese_slices = 7;
        assert_eq!(
            Solution::num_of_burgers(tomato_slices, cheese_slices),
            vec![1, 6]
        );
        assert_eq!(
            Solution::num_of_burgers(17, 4),
            vec![]
        );
        assert_eq!(
            Solution::num_of_burgers(4, 17),
            vec![]
        );
    }
}