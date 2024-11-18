/**
 * [76. Minimum Window Substring](https://leetcode.com/problems/minimum-window-substring/description/)
 */
pub struct Solution;

use std::collections::HashMap;

#[allow(dead_code)]
impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        Solution::min_window_v2(s, t)
    }
    pub fn min_window_v1(s: String, t: String) -> String {
        let mut counts = HashMap::new();
        let mut t_size = 0usize;
        for ch in t.chars() {
            *counts.entry(ch).or_insert(0) += 1;
            t_size += 1;
        }
        let s_chars = s.chars().collect::<Vec<_>>();
        let s_size = s_chars.len();
        let mut left = 0;
        while left < s_size && !counts.contains_key(&s_chars[left]) {
            left += 1;
        }
        let mut min_pos = (left, s_size);
        let mut right = left;
        let mut has_one = false;
        while right < s_size {
            if counts.values().any(|count| *count > 0) {
                *counts.get_mut(&s_chars[right]).unwrap() -= 1;
            }
            while counts.values().all(|count| *count <= 0) {
                has_one = true;
                // 连续几个字母组成的肯定是最小的
                if right == t_size + left - 1 {
                    return s[left..right + 1].to_string();
                }
                if right < min_pos.1 - min_pos.0 + left - 1 {
                    min_pos = (left, right + 1);
                }
                *counts.get_mut(&s_chars[left]).unwrap() += 1;
                left += 1;
                while !counts.contains_key(&s_chars[left]) {
                    left += 1;
                }
            }
            right += 1;
            while right < s_size && !counts.contains_key(&s_chars[right]) {
                right += 1;
            }
        }
        if !has_one {
            return String::new();
        }
        s[min_pos.0..min_pos.1].to_string()
    }

    /**
     * 在题目限定只有字母的前提下，将各字母映射到0-51的usize，用定长数组替代HashMap
     */
    pub fn min_window_v2(s: String, t: String) -> String {
        let mut counts = [0; 52];
        let mut t_size = 0;
        let mut exist = [false; 52];
        for ch in t.bytes() {
            let idx = if ch >= 97 {
                (ch - 71) as usize
            } else {
                (ch - 65) as usize
            };
            counts[idx] += 1;
            t_size += 1;
            exist[idx] = true;
        }
        let s_bytes = s
            .bytes()
            .map(|ch| {
                if ch >= 97 {
                    (ch - 71) as usize
                } else {
                    (ch - 65) as usize
                }
            })
            .collect::<Vec<_>>();
        let s_size = s_bytes.len();
        let mut left = 0;

        while left < s_size && !exist[s_bytes[left]] {
            left += 1;
        }

        let mut min_pos = (left, s_size);
        let mut right = left;
        let mut has_one = false;
        let mut all_included = counts.iter().all(|count| *count <= 0);
        while right < s_size {
            if !all_included {
                counts[s_bytes[right]] -= 1;
                all_included = counts.iter().all(|count| *count <= 0);
            }
            while all_included {
                has_one = true;
                // 连续几个字母组成的肯定是最小的
                if right == t_size + left - 1 {
                    return s[left..right + 1].to_string();
                }
                if right < min_pos.1 - min_pos.0 + left - 1 {
                    min_pos = (left, right + 1);
                }
                counts[s_bytes[left]] += 1;
                if counts[s_bytes[left]] > 0 {
                    all_included = false;
                }
                left += 1;
                while !exist[s_bytes[left]] {
                    left += 1;
                }
            }
            right += 1;
            while right < s_size && !exist[s_bytes[right]] {
                right += 1;
            }
        }
        if !has_one {
            return String::new();
        }
        s[min_pos.0..min_pos.1].to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case1() {
        assert_eq!(
            Solution::min_window("ADOBECODEBANC".to_string(), "ABC".to_string()),
            "BANC".to_string()
        );
    }

    #[test]
    fn test_case2() {
        assert_eq!(
            Solution::min_window("a".to_string(), "a".to_string()),
            "a".to_string()
        );
    }

    #[test]
    fn test_case3() {
        assert_eq!(
            Solution::min_window("a".to_string(), "aa".to_string()),
            "".to_string()
        );
    }

    #[test]
    fn test_case4() {
        assert_eq!(
            Solution::min_window("bba".to_string(), "ab".to_string()),
            "ba".to_string()
        );
    }

    #[test]
    fn test_case5() {
        assert_eq!(
            Solution::min_window("acbdbaab".to_string(), "aabd".to_string()),
            "dbaa".to_string()
        );
    }
}
