struct Solution;

use std::cmp::Ordering;

impl Solution {
    #[allow(dead_code)]
    pub fn full_bloom_flowers(flowers: Vec<Vec<i32>>, people: Vec<i32>) -> Vec<i32> {
        //硬来
        // let mut full = vec![0; 10000000000];
        // for flower in flowers {
        //     for i in flower[0]..=flower[1] {
        //         full[i as usize] += 1;
        //     }
        // }
        // people.iter().map(|x| full[*x as usize]).collect()
        //硬上
        // people.iter().map(|p| {
        //     let mut n = 0;
        //     for flower in &flowers {
        //         if p < &flower[0] || p > &flower[1] {
        //             continue;
        //         }
        //         n += 1;
        //     }
        //     n
        // }).collect()
        // 二分
        let mut bg: Vec<_> = flowers.iter().map(|v| v[0]).collect();
        let mut ed: Vec<_> = flowers.iter().map(|v| v[1]).collect();
        bg.sort_unstable();
        ed.sort_unstable();

        people
            .into_iter()
            .map(|p| {
                let a = bg
                    .binary_search_by(|&x| match x.cmp(&p) {
                        Ordering::Equal => Ordering::Less,
                        ord => ord,
                    })
                    .unwrap_err();
                let b = ed
                    .binary_search_by(|&x| match x.cmp(&p) {
                        Ordering::Equal => Ordering::Greater,
                        ord => ord,
                    })
                    .unwrap_err();
                (a - b) as i32
            })
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test() {
        let flowers = vec![vec![1, 6], vec![3, 7], vec![9, 12], vec![4, 13]];
        let people = vec![2, 3, 7, 11];
        let res = Solution::full_bloom_flowers(flowers, people);
        assert_eq!(vec![1, 2, 2, 2], res);
    }
}
