struct Sollution;

impl Sollution {
    // https://leetcode.cn/problems/count-pairs-that-form-a-complete-day-ii/description/?envType=daily-question&envId=Invalid%20Date
    #[allow(dead_code)]
    pub fn count_complete_day_pairs(hours: Vec<i32>) -> i64 {
        let mut res = 0;
        let mut cnt = vec![0; 24];
        for hour in hours {
            res += cnt[(24 - hour % 24) as usize % 24] as i64;
            cnt[hour as usize % 24] += 1;
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3185() {
        assert_eq!(Sollution::count_complete_day_pairs(vec![12,12,30,24,24]), 2);
        assert_eq!(Sollution::count_complete_day_pairs(vec![72,48,24,3]), 3);

    }
}