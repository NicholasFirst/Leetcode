struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn defang_i_paddr(address: String) -> String {
        address.replace(".", "[.]")
    }
}

#[cfg(test)]
mod test{
    use super::Solution;

    #[test]
    fn test() {
        let address = "192.168.0.193".to_string();
        let res = "192[.]168[.]0[.]193".to_string();
        let result = Solution::defang_i_paddr(address);
        assert_eq!(res, result);
    }
}