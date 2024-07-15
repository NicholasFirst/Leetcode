use std::collections::{HashMap, HashSet};

struct Solution;

#[allow(dead_code)]
impl Solution {
    //https://leetcode.cn/problems/accounts-merge/?envType=daily-question&envId=2024-07-15
    pub fn accounts_merge(accounts: Vec<Vec<String>>) -> Vec<Vec<String>> {
        let mut email_to_idx = HashMap::new();
        for (i, account) in accounts.iter().enumerate() {
            for email in account.iter().skip(1) {
                email_to_idx.entry(email.clone()).or_insert_with(Vec::new).push(i);
            }
        }

        fn dfs(i: usize, accounts: &Vec<Vec<String>>, email_to_idx: &HashMap<String, Vec<usize>>, vis: &mut Vec<bool>, email_set: &mut HashSet<String>) {
            vis[i] = true;
            for email in accounts[i].iter().skip(1) {
                if email_set.contains(email) {
                    continue;
                }
                email_set.insert(email.clone());
                for &j in email_to_idx.get(email).unwrap() { // 遍历所有包含该邮箱地址的账户下标 j
                    if !vis[j] { // j 没有访问过
                        dfs(j, accounts, email_to_idx, vis, email_set);
                    }
                }
            }
        }

        let mut ans = vec![];
        let mut vis = vec![false; accounts.len()];
        for (i, account) in accounts.iter().enumerate() {
            if vis[i] {
                continue;
            }
            let mut email_set = HashSet::new(); // 用于收集 DFS 中访问到的邮箱地址
            dfs(i, &accounts, &email_to_idx, &mut vis, &mut email_set);

            let mut res = email_set.into_iter().collect::<Vec<_>>();
            res.sort_unstable();
            res.insert(0, account[0].clone());

            ans.push(res);
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_721() {
        let accounts = vec![vec!["John".to_string(), "johnsmith@mail.com".to_string(), "john00@mail.com".to_string()], vec!["John".to_string(), "johnnybravo@mail.com".to_string()], vec!["John".to_string(), "johnsmith@mail.com".to_string(), "john_newyork@mail.com".to_string()], vec!["Mary".to_string(), "mary@mail.com".to_string()]];
        assert_eq!(Solution::accounts_merge(accounts), vec![vec!["John".to_string(), "john00@mail.com".to_string(), "john_newyork@mail.com".to_string(), "johnsmith@mail.com".to_string()],  vec!["John".to_string(), "johnnybravo@mail.com".to_string()], vec!["Mary".to_string(), "mary@mail.com".to_string()]]);
        
    }
}