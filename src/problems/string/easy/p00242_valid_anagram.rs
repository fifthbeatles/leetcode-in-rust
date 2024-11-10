/**
 * [242. Valid Anagram](https://leetcode.com/problems/valid-anagram/description/)
 */
pub struct Solution;

use std::collections::HashMap;

#[allow(dead_code)]
impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        Self::is_anagram_v2(s, t)
    }

    pub fn is_anagram_v1(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }
        let mut map = HashMap::new();
        s.chars().zip(t.chars()).for_each(|(c1, c2)| {
            *map.entry(c1).or_insert(0) += 1;
            *map.entry(c2).or_insert(0) -= 1;
        });
        map.values().all(|v| *v == 0)
    }

    pub fn is_anagram_v2(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }
        let mut map = HashMap::new();
        for c in s.chars() {
            *map.entry(c).or_insert(0) += 1;
        }
        for c in t.chars() {
            let count = map.entry(c).or_insert(0);
            *count -= 1;
            if *count < 0 {
                return false;
            }
        }
        map.values().all(|v| *v == 0)
    }

    pub fn is_anagram_v3(s: String, t: String) -> bool {
        let mut v1: Vec<char> = s.chars().collect();
        v1.sort();
        let mut v2: Vec<char> = t.chars().collect();
        v2.sort();
        (v1.len() == v2.len()) && v1.iter().zip(v2.iter()).all(|(v1, v2)| *v1 == *v2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case1() {
        assert_eq!(
            Solution::is_anagram("anagram".to_string(), "nagaram".to_string()),
            true
        );
        assert_eq!(
            Solution::is_anagram("rat".to_string(), "car".to_string()),
            false
        );
        assert_eq!(
            Solution::is_anagram("a".to_string(), "aa".to_string()),
            false
        );
    }
}
