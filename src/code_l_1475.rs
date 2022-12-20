
struct Solution;

impl Solution{

    #[allow(dead_code)]
    // https://leetcode.cn/problems/final-prices-with-a-special-discount-in-a-shop/
    pub fn final_prices(prices: Vec<i32>) -> Vec<i32> {
        if prices.len() <= 1 {
            return prices;
        }
        let mut res:Vec<i32> = Vec::with_capacity(prices.len());
        for i in 0..prices.len(){
            let target = prices[i];
            if i + 1 != res.len() {
                for j in i + 1..prices.len() {
                    if target >= prices[j] {
                        res.push(target - prices[j]);
                        break;
                    }
                }    
            }
            if let None = res.get(i) {
                res.push(target);
            }
        }
        res
    }
}

#[cfg(test)]
mod test{
    use super::Solution;

    #[test]
    fn test() {
        let prices = vec![8,4,6,2,3];
        let res = Solution::final_prices(prices);
        assert_eq!(vec![4,2,4,2,3], res);
    }

    #[test]
    fn test1() {
        let prices = vec![1,2,3,4,5];
        let res = Solution::final_prices(prices);
        assert_eq!(vec![1,2,3,4,5], res);
    }

    #[test]
    fn test2() {
        let prices = vec![10,1,1,6];
        let res = Solution::final_prices(prices);
        assert_eq!(vec![9,0,1,6], res);
    }
}