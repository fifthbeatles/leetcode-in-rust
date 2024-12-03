/**
 * [3365. Rearrange K Substrings to Form Target String](https://leetcode.com/problems/rearrange-k-substrings-to-form-target-string/description/)
 */
pub struct Solution;

use std::collections::HashMap;

#[allow(dead_code)]
impl Solution {
    pub fn is_possible_to_rearrange(s: String, t: String, k: i32) -> bool {
        Solution::is_possible_to_rearrange_v2(s, t, k)
    }

    pub fn is_possible_to_rearrange_v1(s: String, t: String, k: i32) -> bool {
        let size = s.len();
        let k = k as usize;
        if (t.len() != size) || size % k != 0 {
            return false;
        }
        let step = size / k;
        let mut counts = HashMap::new();
        for start in (0..size).step_by(step) {
            let k1 = &s[start..start + step];
            let k2 = &t[start..start + step];
            *counts.entry(k1).or_insert(0) += 1;
            *counts.entry(k2).or_insert(0) -= 1;
        }
        counts.values().all(|v| *v == 0)
    }

    pub fn is_possible_to_rearrange_v2(s: String, t: String, k: i32) -> bool {
        let size = s.len();
        let k = k as usize;
        if (t.len() != size) || size % k != 0 {
            return false;
        }
        let step = size / k;
        let mut counts = HashMap::new();
        for start in (0..size).step_by(step) {
            let key = &s[start..start + step];
            *counts.entry(key).or_insert(0) += 1;
        }
        for start in (0..size).step_by(step) {
            let key: &str = &t[start..start + step];
            let c = counts.entry(key).or_insert(0);
            if *c <= 0 {
                return false;
            }
            *c -= 1;
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case1() {
        assert_eq!(
            Solution::is_possible_to_rearrange("abcd".to_string(), "cdab".to_string(), 2),
            true
        );
    }

    #[test]
    fn test_case2() {
        assert_eq!(
            Solution::is_possible_to_rearrange("aabbcc".to_string(), "bbaacc".to_string(), 3),
            true
        );
    }

    #[test]
    fn test_case3() {
        assert_eq!(
            Solution::is_possible_to_rearrange("aabbcc".to_string(), "bbaacc".to_string(), 2),
            false
        );
    }
}
