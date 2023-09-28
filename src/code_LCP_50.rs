struct Solution;

impl Solution {
    //https://leetcode.cn/problems/WHnhjV/description/?envType=daily-question&envId=2023-09-28
    #[allow(dead_code)]
    pub fn give_gem(mut gem: Vec<i32>, operations: Vec<Vec<i32>>) -> i32 {
        for op in operations {
            let num = gem[op[0] as usize] / 2;
            gem[op[0] as usize] -= num;
            gem[op[1] as usize] += num;
        }
        gem.iter().max().unwrap() - gem.iter().min().unwrap()
    }
}

#[cfg(test)]
mod test{
    use super::Solution;

    #[test]
    fn test() {
        let gem = vec![3,1,2];
        let operations = vec![vec![0,2],vec![2,1],vec![2,0]];
        let res = Solution::give_gem(gem, operations);
        assert_eq!(2, res);
    }

    #[test]
    fn test1() {
        let gem = vec![100,0,50,100];
        let operations = vec![vec![0,2],vec![0,1],vec![3,0],vec![3,0]];
        let res = Solution::give_gem(gem, operations);
        assert_eq!(75, res);
    } 
}