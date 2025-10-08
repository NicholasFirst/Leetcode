struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn successful_pairs(mut spells: Vec<i32>, potions: Vec<i32>, success: i64) -> Vec<i32> {
        let mx = *potions.iter().max().unwrap() as usize;
        let mut cnt = vec![0; mx + 1];
        for y in potions {
            cnt[y as usize] += 1; // 统计每种药水的出现次数
        }

        // 计算 cnt 的后缀和
        for i in (0..mx).rev() {
            cnt[i] += cnt[i + 1];
        }
        // 计算完毕后，cnt[i] 就是 potions 值 >= i 的药水个数

        let success = success as usize;
        for x in spells.iter_mut() {
            let low = (success - 1) / *x as usize + 1;
            *x = if low <= mx { cnt[low] } else { 0 };
        }
        spells
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2300() {
        assert_eq!(Solution::successful_pairs(vec![5, 1, 3], vec![1, 2, 3, 4, 5], 7), vec![4, 0, 3]);
        assert_eq!(Solution::successful_pairs(vec![3, 1, 2], vec![8, 5, 8], 16), vec![2, 0, 2]);
    }
}