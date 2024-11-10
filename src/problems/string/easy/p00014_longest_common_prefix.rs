/**
 * [14. Longest Common Prefix](https://leetcode.com/problems/longest-common-prefix/description/)
 */
pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        strs.into_iter()
            .reduce(|acc, cur| {
                acc.chars()
                    .zip(cur.chars())
                    .take_while(|v| v.0 == v.1)
                    .map(|v| v.0)
                    .collect()
            })
            .unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case1() {
        let strs = vec![
            "flower".to_string(),
            "flow".to_string(),
            "flight".to_string(),
        ];
        assert_eq!(Solution::longest_common_prefix(strs), "fl".to_string());
    }

    #[test]
    fn test_case2() {
        let strs = vec!["dog".to_string(), "rececar".to_string(), "car".to_string()];
        assert_eq!(Solution::longest_common_prefix(strs), "".to_string());
    }
}
