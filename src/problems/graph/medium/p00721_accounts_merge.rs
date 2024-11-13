/**
 * [721. Accounts Merge](https://leetcode.com/problems/accounts-merge/description/)
 */
pub struct Solution;

use crate::data_structures::union_find::UnionFind;

use std::collections::BTreeMap;
use std::collections::HashMap;

#[allow(dead_code)]
impl Solution {
    pub fn accounts_merge(accounts: Vec<Vec<String>>) -> Vec<Vec<String>> {
        Solution::accounts_merge_v2(accounts)
    }
    pub fn accounts_merge_v1(accounts: Vec<Vec<String>>) -> Vec<Vec<String>> {
        let mut uf = UnionFind::new(accounts.len());
        let mut emails = HashMap::new();
        for (i, account) in accounts.iter().enumerate() {
            for email in account.iter().skip(1) {
                if let Some(&j) = emails.get(email) {
                    uf.union(i, j);
                } else {
                    emails.insert(email, i);
                }
            }
        }
        let mut name_and_emails = HashMap::new();
        for (&email, &i) in &emails {
            let parent = uf.find(i);
            name_and_emails
                .entry(parent)
                .or_insert(vec![accounts[parent][0].clone()])
                .push(email.clone());
        }
        let mut ret = Vec::new();
        for (_, emails) in name_and_emails.iter_mut() {
            emails[1..].sort();
            ret.push(emails.clone());
        }
        ret
    }

    pub fn accounts_merge_v2(accounts: Vec<Vec<String>>) -> Vec<Vec<String>> {
        let mut uf = UnionFind::new(accounts.len());
        // 这里使用BTreeMap，邮件就是按顺序的，后面就可以不用再排序了
        let mut emails = BTreeMap::new();
        for (i, account) in accounts.iter().enumerate() {
            for email in account.iter().skip(1) {
                if let Some(&j) = emails.get(email) {
                    uf.union(i, j);
                } else {
                    emails.insert(email, i);
                }
            }
        }
        // 这里使用BTreeMap主要是希望遍历时能保持各账号原来的顺序，方便写测试用例，题目里是允许几个账号之间的顺序变化的，所以也可以用HashMap
        let mut name_and_emails = BTreeMap::new();
        for (&email, &i) in &emails {
            let parent = uf.find(i);
            name_and_emails
                .entry(parent)
                .or_insert(vec![accounts[parent][0].clone()])
                .push(email.clone());
        }
        let mut ret = Vec::new();
        for (_, emails) in name_and_emails.iter_mut() {
            ret.push(emails.clone());
        }
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case1() {
        let accounts = vec![
            vec!["John", "johnsmith@mail.com", "john_newyork@mail.com"],
            vec!["John", "johnsmith@mail.com", "john00@mail.com"],
            vec!["Mary", "mary@mail.com"],
            vec!["John", "johnnybravo@mail.com"],
        ]
        .into_iter()
        .map(|v| v.into_iter().map(|s| s.to_string()).collect::<Vec<_>>())
        .collect();
        let target: Vec<Vec<String>> = vec![
            vec![
                "John",
                "john00@mail.com",
                "john_newyork@mail.com",
                "johnsmith@mail.com",
            ],
            vec!["Mary", "mary@mail.com"],
            vec!["John", "johnnybravo@mail.com"],
        ]
        .into_iter()
        .map(|v| v.into_iter().map(|s| s.to_string()).collect::<Vec<_>>())
        .collect();
        let ret = Solution::accounts_merge(accounts);
        assert_eq!(ret, target);
    }

    #[test]
    fn test_case2() {
        let accounts = vec![
            vec!["Gabe", "Gabe0@m.co", "Gabe3@m.co", "Gabe1@m.co"],
            vec!["Kevin", "Kevin3@m.co", "Kevin5@m.co", "Kevin0@m.co"],
            vec!["Ethan", "Ethan5@m.co", "Ethan4@m.co", "Ethan0@m.co"],
            vec!["Hanzo", "Hanzo3@m.co", "Hanzo1@m.co", "Hanzo0@m.co"],
            vec!["Fern", "Fern5@m.co", "Fern1@m.co", "Fern0@m.co"],
        ]
        .into_iter()
        .map(|v| v.into_iter().map(|s| s.to_string()).collect::<Vec<_>>())
        .collect();
        let target: Vec<Vec<String>> = vec![
            vec!["Gabe", "Gabe0@m.co", "Gabe1@m.co", "Gabe3@m.co"],
            vec!["Kevin", "Kevin0@m.co", "Kevin3@m.co", "Kevin5@m.co"],
            vec!["Ethan", "Ethan0@m.co", "Ethan4@m.co", "Ethan5@m.co"],
            vec!["Hanzo", "Hanzo0@m.co", "Hanzo1@m.co", "Hanzo3@m.co"],
            vec!["Fern", "Fern0@m.co", "Fern1@m.co", "Fern5@m.co"],
        ]
        .into_iter()
        .map(|v| v.into_iter().map(|s| s.to_string()).collect::<Vec<_>>())
        .collect();
        let ret = Solution::accounts_merge(accounts);
        assert_eq!(ret, target);
    }

    #[test]
    fn test_case3() {
        let accounts = vec![
            vec!["David", "David0@m.co", "David1@m.co"],
            vec!["David", "David3@m.co", "David4@m.co"],
            vec!["David", "David4@m.co", "David5@m.co"],
            vec!["David", "David2@m.co", "David3@m.co"],
            vec!["David", "David1@m.co", "David2@m.co"],
        ]
        .into_iter()
        .map(|v| v.into_iter().map(|s| s.to_string()).collect::<Vec<_>>())
        .collect();
        let target: Vec<Vec<String>> = vec![vec![
            "David",
            "David0@m.co",
            "David1@m.co",
            "David2@m.co",
            "David3@m.co",
            "David4@m.co",
            "David5@m.co",
        ]]
        .into_iter()
        .map(|v| v.into_iter().map(|s| s.to_string()).collect::<Vec<_>>())
        .collect();
        let ret = Solution::accounts_merge(accounts);
        assert_eq!(ret, target);
    }
}
