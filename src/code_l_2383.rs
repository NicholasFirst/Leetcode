struct Solution;

impl Solution {

    // https://leetcode.cn/problems/minimum-hours-of-training-to-win-a-competition/
    #[allow(dead_code)]
    pub fn min_number_of_hours(initial_energy: i32, mut initial_experience: i32, energy: Vec<i32>, experience: Vec<i32>) -> i32 {
        let sum = energy.iter().sum::<i32>();
        let mut ans = std::cmp::max(0 , sum - initial_energy  + 1);

        for i in 0..experience.len(){
            if initial_experience <= experience[i] {
                ans += experience[i] - initial_experience + 1;
                initial_experience += experience[i] - initial_experience + 1;
            }
            initial_experience += experience[i];
        }
        ans
    }
}


#[cfg(test)]
mod test{
    use super::Solution;

    #[test]
    fn test() {
        let initial_energy = 5;
        let initial_experience = 3; 
        let energy = vec![1,4,3,2];
        let experience = vec![2,6,3,1];
        let ans  = Solution::min_number_of_hours(initial_energy, initial_experience, energy, experience);
        assert_eq!(ans, 8);
    }
}