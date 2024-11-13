/**
 * [139. Word Break](https://leetcode.com/problems/word-break/description/)
 */
pub struct Solution;

use std::collections::HashSet;

#[allow(dead_code)]
impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        let length = s.len();
        let mut dp = vec![false; length + 1];
        dp[0] = true;
        let word_dict: HashSet<_> = word_dict.into_iter().collect();
        for i in 1..=length {
            for j in 0..i {
                if dp[j] && word_dict.contains(&s[j..i]) {
                    dp[i] = true;
                    break;
                }
            }
        }
        dp[length]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case1() {
        let word_dict = vec!["leet", "code"]
            .into_iter()
            .map(|s| s.to_string())
            .collect();
        assert_eq!(
            Solution::word_break("leetcode".to_string(), word_dict),
            true
        );
    }

    #[test]
    fn test_case2() {
        let word_dict = vec!["apple", "pen"]
            .into_iter()
            .map(|s| s.to_string())
            .collect();
        assert_eq!(
            Solution::word_break("applepenapple".to_string(), word_dict),
            true
        );
    }

    #[test]
    fn test_case3() {
        let word_dict = vec!["cats", "dog", "sand", "and", "cat"]
            .into_iter()
            .map(|s| s.to_string())
            .collect();
        assert_eq!(
            Solution::word_break("catsandog".to_string(), word_dict),
            false
        );
    }

    #[test]
    fn test_case4() {
        let word_dict = vec![
            "a",
            "aa",
            "aaa",
            "aaaa",
            "aaaaa",
            "aaaaaa",
            "aaaaaaa",
            "aaaaaaaa",
            "aaaaaaaaa",
            "aaaaaaaaaa",
        ]
        .into_iter()
        .map(|s| s.to_string())
        .collect();
        assert_eq!(
            Solution::word_break("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaab".to_string(), word_dict),
            false
        );
    }
}
