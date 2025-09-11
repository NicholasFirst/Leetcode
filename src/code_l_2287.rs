struct Solution;

impl Solution {

    // https://leetcode.cn/problems/rearrange-characters-to-make-target-string/
    #[allow(dead_code)]
    pub fn rearrange_characters(s: String, target: String) -> i32 {
        let (mut maps, mut mapt) = ([0_i8; 26], [0_i8; 26]);
        s.bytes().for_each(|c| maps[(c - b'a') as usize] += 1);
        target.bytes().for_each(|c| mapt[(c - b'a') as usize] += 1);

        let mut ans = i8::MAX;
        for (idx, &cnt) in mapt.iter().enumerate() {
            if cnt > 0 {
                if maps[idx] < cnt {
                    return 0;
                }
                ans = ans.min(maps[idx] / cnt);
            }
        }
        ans as i32
    }
}

#[cfg(test)]
mod test{
    use super::Solution;

    #[test]
    fn test() {
        let s = "abbaccaddaeea".to_string();
        let target = "aaaaa".to_string();
        let ans = Solution::rearrange_characters(s, target);
        assert_eq!(ans, 1);
    }

    #[test]
    fn test1() {
        let s = "rav".to_string();
        let target = "vr".to_string();
        let ans = Solution::rearrange_characters(s, target);
        assert_eq!(ans, 1);
    }
}